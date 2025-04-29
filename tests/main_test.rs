use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use base64::encode;
use dotenv::dotenv;
use std::env;

#[test]
fn it_works() {
    dotenv().ok();

    // Ambil key dari env
    let key_str = env::var("AES_KEY").expect("AES_KEY not set in .env");



    // Buat key
    assert_eq!(key_str.len(), 32, "AES_KEY must be exactly 32 characters");
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let cipher = Aes256Gcm::new(key);

    // Nonce random (12 byte)
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    println!("{:?}", encode(&nonce));
    // Plaintext
    let plaintext = b"Data rahasia dari Rust";

    // Enkripsi
    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).expect("Gagal enkripsi");
    let password = encode([nonce.as_slice(),ciphertext.as_slice()].concat());
    println!("Ciphertext (hex): {}", hex::encode(&ciphertext));

// decode
    let data = base64::decode(password).unwrap();
    let (nonce_bytes, ciphertext) = data.split_at(12); // AES-GCM nonce = 12 bytes
    let nonce = Nonce::from_slice(nonce_bytes);
    let plain = cipher.decrypt(nonce, ciphertext).unwrap();

    println!("Decrypted: {}", String::from_utf8(plain).unwrap());
}