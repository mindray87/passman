use aes::Aes128;
use block_modes::{BlockMode, Cbc, InvalidKeyIvLength};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;
type Result<T> = std::result::Result<T, String>;

pub fn encrypt(text: &String, key: &String, initial_vector: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes128Cbc::new_var(&key.as_bytes(), &initial_vector).unwrap();
    Ok(cipher.encrypt_vec(text.as_bytes()))
}

pub fn decrypt(ciphertext: &Vec<u8>, key: &String, initial_vector: &[u8]) -> Result<String> {
    let cipher = Aes128Cbc::new_var(&key.as_bytes(), &initial_vector).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext.as_slice()).unwrap();
    String::from_utf8(decrypted_ciphertext).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use aes::Aes128;
    use block_modes::{BlockMode, Cbc};
    use block_modes::block_padding::Pkcs7;
    use hex_literal::hex;
    use rand::RngCore;
    use rand::rngs::OsRng;

    use crate::passman_crypto::{decrypt, encrypt};

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    #[test]
    fn test() {
        let key = "aaaaaaaaaaaaaaaa".to_string();
        let mut iv = [0 as u8; 16];
        OsRng.fill_bytes(&mut iv);
        let plaintext = "Hello world!".to_string();

        let ciphertext = encrypt(&plaintext, &key, &iv).unwrap();
        let decrypted_ciphertext = decrypt(&ciphertext, &key, &iv).unwrap();
        assert_eq!(decrypted_ciphertext, plaintext);
    }
}
