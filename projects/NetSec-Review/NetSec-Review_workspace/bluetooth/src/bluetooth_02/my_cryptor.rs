use aes_gcm::{
    aead::{Aead, Generate, Key, KeyInit},
    Aes256Gcm, Nonce,
};

// From Rust aes_gcm crate
// AES-GCM with a 256-bit key and 96-bit nonce.

pub fn encrypt(
    key: &Key<Aes256Gcm>,
    plaintext: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let cipher = Aes256Gcm::new(key);

    // Every encryption with same key needs a new nonce
    // nonce = number used once
    let nonce = Nonce::generate();

    // cipher uses nonce
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes())?;

    // nonce is prepended to message
    let mut output = nonce.to_vec();
    output.extend_from_slice(&ciphertext);
    Ok(output)
}

pub fn decrypt(key: &Key<Aes256Gcm>, data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    const NONCE_LEN: usize = 12; //96-bit nonce

    if data.len() < NONCE_LEN {
        return Err("input is too short".into());
    }

    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::try_from(&data[..NONCE_LEN])?;
    let plaintext = cipher.decrypt(&nonce, &data[NONCE_LEN..])?;

    Ok(String::from_utf8(plaintext)?)
}
