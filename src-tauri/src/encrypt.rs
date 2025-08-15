pub fn encrypt(_data: &str, _secret_key: &str) -> Result<String, ()> {
    todo!()
}
pub fn decrypt(_encrypted_data: &str, _secret_key: &str) -> Result<String, ()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let test_data = "test data";
        let test_key = "test key";
        let test_encrypted_data = encrypt(test_data, test_key).expect("failed encryption");
        let _ = decrypt(&test_encrypted_data, test_key);
    }
}
