// https://github.com/DaGenix/rust-crypto/blob/master/examples/symmetriccipher.rs

extern crate crypto;
extern crate rand;

use crypto::{aes, blockmodes, buffer, symmetriccipher};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};

// AES-256/CBC/Pkcs encryption.
fn aes_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = (encryptor.encrypt(&mut read_buffer, &mut write_buffer, true))?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

// AES-256/CBC/Pkcs encryption.
fn aes_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

pub fn encrypt(text: &String, key: &String, initial_vector: &[u8; 16]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    return aes_encrypt(text.as_bytes(), key.as_bytes(), initial_vector);
}

pub fn decrypt(text: &Vec<u8>, key: &String, initial_vector: &[u8; 16]) -> Result<String, symmetriccipher::SymmetricCipherError> {
    return aes_decrypt(text.as_slice(), key.as_bytes(), initial_vector)
        .map(|vector| String::from(std::str::from_utf8(vector.as_slice()).unwrap()));
}

#[cfg(test)]
mod tests {
    use rand::RngCore;
    use rand::rngs::OsRng;

    use crate::{decrypt, encrypt};
    use std::env;

    #[test]
    fn en_and_decryption() {
        env::set_var("RUST_BACKTRACE", "1");

        let message = String::from("this text is top secret");
        let key = String::from("keeeey");
        let mut iv: [u8; 16] = [0; 16];
        OsRng.fill_bytes(&mut iv);


        let encrypted = match encrypt(&message, &key, &iv) {
            Ok(s) => s,
            Err(_) => panic!("Encryption failed."),
        };

        let decrypt = match decrypt(&encrypted, &key, &iv) {
            Ok(s) => s,
            Err(_) => panic!("Decryption failed."),
        };

        assert_eq!(decrypt, message)
    }
}
