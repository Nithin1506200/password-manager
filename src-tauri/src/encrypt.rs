#![allow(dead_code)]
use base64::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
mod test;

// https://docs.rs/chacha20poly1305/latest/chacha20poly1305/
pub fn generate_key() -> String {
    KeyStr::new().get_base64()
}
#[derive(Debug)]
struct KeyStr(Key);
const KEY_LENGTH: usize = 32;

#[derive(Debug)]
struct NonceStr(Nonce);
const NONCE_LENGTH: usize = 12;

impl KeyStr {
    pub fn from_base64(value: &str) -> Result<KeyStr, String> {
        let key_slice = BASE64_STANDARD
            .decode(value)
            .map_err(|_| "Invalid Base64 string".to_string())?;
        if key_slice.len() == KEY_LENGTH {
            let key = Key::from_slice(&key_slice);
            Ok(KeyStr(*key))
        } else {
            Err("key length should be 32".to_string())
        }
    }
    pub fn get_base64(&self) -> String {
        let KeyStr(key) = self;
        BASE64_STANDARD.encode(key.as_slice())
    }
    pub fn new() -> KeyStr {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        KeyStr(key)
    }
    pub fn peak(&self) -> Key {
        let KeyStr(key) = self;
        *key
    }
}

// Custom serialization for KeyStr
impl Serialize for KeyStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the KeyStr as a base64 string
        serializer.serialize_str(&self.get_base64())
    }
}

// Custom deserialization for KeyStr
impl<'de> Deserialize<'de> for KeyStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Define a visitor that expects a string
        struct KeyStrVisitor;

        impl<'de> Visitor<'de> for KeyStrVisitor {
            type Value = KeyStr;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a base64 encoded string representing a Key")
            }

            fn visit_str<E>(self, value: &str) -> Result<KeyStr, E>
            where
                E: de::Error,
            {
                KeyStr::from_base64(value).map_err(de::Error::custom)
            }
        }

        // Use the visitor to deserialize the value
        deserializer.deserialize_str(KeyStrVisitor)
    }
}

impl NonceStr {
    pub fn peak(&self) -> Nonce {
        let NonceStr(key) = self;
        *key
    }
    pub fn try_from_string(value: String) -> Result<Self, String> {
        let key_slice = BASE64_STANDARD
            .decode(value)
            .map_err(|_| "Invalid Base64 string".to_string())?;
        if key_slice.len() == NONCE_LENGTH {
            let key = Nonce::from_slice(&key_slice);
            Ok(NonceStr(*key))
        } else {
            Err("key length should be 12".to_string())
        }
    }
    pub fn get_base64(&self) -> String {
        let NonceStr(key) = self;
        BASE64_STANDARD.encode(key.as_slice())
    }
    pub fn new() -> NonceStr {
        let key = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        NonceStr(key)
    }
}

// Custom serialization for NonceStr
impl Serialize for NonceStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the NonceStr as a base64 string
        serializer.serialize_str(&self.get_base64())
    }
}

// Custom deserialization for NonceStr
impl<'de> Deserialize<'de> for NonceStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Define a visitor that expects a string
        struct NonceStrVisitor;

        impl<'de> Visitor<'de> for NonceStrVisitor {
            type Value = NonceStr;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a base64 encoded string representing a Nonce")
            }

            fn visit_str<E>(self, value: &str) -> Result<NonceStr, E>
            where
                E: de::Error,
            {
                NonceStr::try_from_string(value.to_string()).map_err(de::Error::custom)
            }
        }

        // Use the visitor to deserialize the value
        deserializer.deserialize_str(NonceStrVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedData {
    data: String,
    nonce: NonceStr,
    version: String,
    time: String,
}

impl EncryptedData {
    pub fn encrypt<T>(key: &str, data: T) -> Result<Self, String>
    where
        T: AsRef<str> + From<String>,
    {
        let data = data.as_ref();
        let key = KeyStr::from_base64(key)?.peak();
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = NonceStr::new();
        let data = cipher.encrypt(&nonce.peak(), data.as_bytes()).map_or_else(
            |err| Err(err.to_string()),
            |e| Ok(BASE64_STANDARD.encode(e)),
        )?;
        Ok(EncryptedData {
            data,
            nonce: nonce,
            time: "".to_string(),
            version: "".to_string(),
        })
    }

    pub fn decrypt<F, T>(&self, get_key: F) -> Result<T, String>
    where
        F: FnOnce(&str) -> Result<String, String>,
        T: AsRef<str> + From<String>,
    {
        let key_str = get_key(&self.version)?;
        let key = KeyStr::from_base64(&key_str)?.peak();
        let cipher = ChaCha20Poly1305::new(&key);
        let actual_text_string: Vec<u8> = BASE64_STANDARD
            .decode(&self.data)
            .map_err(|err| err.to_string())?;
        cipher
            .decrypt(&self.nonce.peak(), actual_text_string.as_ref())
            .map_or_else(
                |err| Err(err.to_string()),
                |e| Ok(String::from_utf8_lossy(&e).to_string().into()),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encrypt_decrypt() {
        let data = "test data".to_string();
        let key = generate_key();
        let enc = EncryptedData::encrypt(&key, data.clone());
        let dec = enc
            .as_ref()
            .unwrap()
            .decrypt::<_, String>(|_| Ok(key.to_string()));
        println!("{:?} {:?}", enc.as_ref(), dec);
        assert_eq!(dec.unwrap(), data);
    }

    #[test]
    fn test_serialization_skips_phantamdata() {
        let data = "test data".to_string();
        let key = generate_key();
        let enc = EncryptedData::encrypt(&key, data.clone()).unwrap();

        // Serialize to JSON
        let json = serde_json::to_string(&enc).unwrap();
        println!("Serialized JSON: {}", json);

        // Deserialize back and verify data integrity
        let deserialized: EncryptedData = serde_json::from_str(&json).unwrap();
        let dec: String = deserialized.decrypt(|_| Ok(key.to_string())).unwrap();
        assert_eq!(dec, data);
    }
}
