#![allow(dead_code)]
use base64::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};

static DELIMITER: &str = "__NONCE__";

fn key_from_string<'a>(key: &'a str) -> Result<&'a Key, &'a str> {
    let key_slice = key.as_bytes();
    if key_slice.len() == 32 {
        let key = Key::from_slice(key_slice);
        Ok(key)
    } else {
        Err("key length should be 32")
    }
}

fn nonce_from_string<'a>(nonce: &'a str) -> Result<Nonce, &'a str> {
    let nonce_slice = nonce.as_bytes();
    if nonce_slice.len() == 12 {
        let nonce = Nonce::from_slice(nonce_slice);
        Ok(*nonce)
    } else {
        Err("nonce length should be 12")
    }
}

fn nonce_from_base64(base64: &str) -> Result<Nonce, String> {
    // Decode the Base64 input
    let decoded_bytes = BASE64_STANDARD
        .decode(base64)
        .map_err(|_| "Invalid Base64 string".to_string())?;
    if decoded_bytes.len() == 12 {
        let nonce = Nonce::from_slice(&decoded_bytes);
        Ok(*nonce)
    } else {
        Err("nonce length should be 12".to_string())
    }
}
fn key_from_base64(base64: &str) -> Result<Key, String> {
    // Decode the Base64 input
    let decoded_bytes = BASE64_STANDARD
        .decode(base64)
        .map_err(|_| "Invalid Base64 string".to_string())?;
    if decoded_bytes.len() == 32 {
        let key = Key::from_slice(&decoded_bytes);
        Ok(*key)
    } else {
        Err("key length should be 32".to_string())
    }
}

pub fn generate_key_base64() -> String {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    BASE64_STANDARD.encode(key.as_slice())
}
pub fn generate_nonce_base64() -> String {
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    BASE64_STANDARD.encode(nonce.as_slice())
}
pub fn encrypt(key_text_base64: &str, cipher_text: &str) -> Result<String, String> {
    let key = key_from_base64(key_text_base64)?;
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce_base_64 = BASE64_STANDARD.encode(nonce.as_slice());
    match cipher.encrypt(&nonce, cipher_text.as_ref()) {
        Ok(txt) => Ok(format!(
            "{}{}{}",
            nonce_base_64,
            DELIMITER,
            BASE64_STANDARD.encode(txt.as_slice())
        )),
        Err(_) => Err("fasdf".to_string()),
    }
}

pub fn decrypt(key_text: &str, encrypted_text: &str) -> Result<String, String> {
    let s: Vec<&str> = encrypted_text.split(DELIMITER).collect();
    let (nonce, actual_text) = match (s.get(0), s.get(1)) {
        (Some(nonce), Some(actual_text)) => Ok((*nonce, *actual_text)),
        _ => Err("worng format text"),
    }?;

    let actual_text_string: Vec<u8> = BASE64_STANDARD
        .decode(actual_text)
        .map_err(|err| err.to_string())?;
    let key = key_from_base64(key_text)?;
    let nonce = nonce_from_base64(nonce)?;
    let cipher = ChaCha20Poly1305::new(&key);
    match cipher.decrypt(&nonce, actual_text_string.as_ref()) {
        Ok(arr) => Ok(String::from_utf8_lossy(&arr).to_string()),
        Err(err) => Err(err.to_string()),
    }
}

#[test]
fn test_init() {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let text = b"fas";
    println!("{:#?} {:#?}", key.len(), nonce.len());
    let ciphertext = cipher.encrypt(&nonce, text.as_ref()).expect("fasd");
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).expect("hkhk");
    assert_eq!(&plaintext, text);
}

#[test]
fn test_init_1() {
    let key_str = generate_key_base64(); //32 length
    let nonce_str = "123456789012"; //12 length
    let binding = BASE64_STANDARD.decode(&key_str).expect("unable to gen key");
    let key = //ChaCha20Poly1305::generate_key(&mut OsRng);
    Key::from_slice(binding.as_ref());
    let nonce =// ChaCha20Poly1305::generate_nonce(&mut OsRng);

    Nonce::from_slice(nonce_str.as_bytes());
    let text = b"ffskladjf lsajfsaklfdj lksajflasjfdljas dlfjaslfj lfdjas";
    let cipher = ChaCha20Poly1305::new(&key);
    let ciphertext = cipher.encrypt(&nonce, text.as_ref()).expect("fasd");
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).expect("hkhk");
    assert_eq!(&plaintext, text);
}
#[test]
fn test_nonce_from_base64() -> Result<(), Box<dyn std::error::Error>> {
    let nonce = "123456789012";
    let base64 = BASE64_STANDARD.encode(nonce.as_bytes());
    let nonce = nonce_from_base64(&base64).expect("array");
    Ok(())
}
#[test]
fn test_init_2() {
    let key_text = generate_key_base64(); //32 length
    let cipher_text = "hellb hi jbow world";
    let encrypt_text = encrypt(&key_text, cipher_text).expect("failed to encrypt");

    let decrypted_text = decrypt(&key_text, &encrypt_text).expect("failed to Decrypt");
    println!(
        "encrypted text :{:#?} decrypted text {:#?}",
        encrypt_text, decrypted_text
    );
    assert_eq!(decrypted_text, cipher_text)
}

#[test]
fn base_64_test() {
    let str = "hellow World";
    let encoded_base64 = BASE64_STANDARD.encode(str.as_bytes());
    println!("encoded : {}", encoded_base64);
    let decoded = String::from_utf8_lossy(
        BASE64_STANDARD
            .decode(encoded_base64)
            .expect("unable to decode nonce")
            .as_ref(),
    )
    .to_string();
    println!("decoded : {}", decoded);
}
