use openssl::symm::{Cipher, Crypter, Mode};
use rand::RngCore;
use hex;

fn key_from_env() -> Vec<u8> {
    let key_hex = std::env::var("KEY").expect("KEY env var required");
    hex::decode(key_hex).expect("invalid hex in KEY")
}

fn cipher_from_env() -> Cipher {
    match std::env::var("ALGORITHM").unwrap_or_else(|_| "aes-256-cbc".into()).as_str() {
        "aes-256-cbc" => Cipher::aes_256_cbc(),
        "aes-192-cbc" => Cipher::aes_192_cbc(),
        "aes-128-cbc" => Cipher::aes_128_cbc(),
        other => panic!("unsupported algorithm: {}", other),
    }
}

pub fn encrypt(text: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let key = key_from_env();
    let cipher = cipher_from_env();

    if key.len() != cipher.key_len() {
        return Err(format!("key length {} does not match cipher key_len {}", key.len(), cipher.key_len()).into());
    }

    let iv_len = cipher.iv_len().unwrap_or(16);
    let mut iv = vec![0u8; iv_len];
    rand::rngs::OsRng.fill_bytes(&mut iv);

    let mut crypter = Crypter::new(cipher, Mode::Encrypt, &key, Some(&iv))?;
    crypter.pad(true);

    let mut out = vec![0u8; text.as_bytes().len() + cipher.block_size()];
    let mut count = crypter.update(text.as_bytes(), &mut out)?;
    count += crypter.finalize(&mut out[count..])?;
    out.truncate(count);

    Ok((hex::encode(iv), hex::encode(out)))
}

pub fn decrypt(iv_hex: &str, encrypted_hex: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = key_from_env();
    let cipher = cipher_from_env();

    if key.len() != cipher.key_len() {
        return Err(format!("key length {} does not match cipher key_len {}", key.len(), cipher.key_len()).into());
    }

    let iv = hex::decode(iv_hex)?;
    let data = hex::decode(encrypted_hex)?;

    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, &key, Some(&iv))?;
    decrypter.pad(true);

    let mut out = vec![0u8; data.len() + cipher.block_size()];
    let mut count = decrypter.update(&data, &mut out)?;
    count += decrypter.finalize(&mut out[count..])?;
    out.truncate(count);

    let plaintext = String::from_utf8(out)?;
    Ok(plaintext)
}

pub fn get_token() -> String {
    let mut buf = [0u8; 64];
    rand::rngs::OsRng.fill_bytes(&mut buf);
    hex::encode(buf)
}