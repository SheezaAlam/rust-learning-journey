use aes_gcm::aead::{Aead, KeyInit}; // Traits for encryption/decryption
use aes_gcm::{Aes256Gcm, Nonce};    // AES-256-GCM cipher + Nonce type
use rand::RngCore;                  // Trait for random numbers
use rand::rngs::OsRng;              // Secure OS-based RNG

fn main() {
    // Generate random 256-bit key (32 bytes)
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).unwrap();

    // Generate random 12-byte nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Message to encrypt
    let plaintext = b"hello protocol world";

    // Encrypt
    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref()).unwrap();
    println!("Ciphertext (hex): {}", hex::encode(&ciphertext));

    // Decrypt
    let decrypted = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    println!("Decrypted: {:?}", String::from_utf8(decrypted).unwrap());
}
/*output
Ciphertext (hex): d6ece83afb72b63184b53fb3a767bbfacfcffe16250cfba133f24c903c3cbeffe3163bef
Decrypted: "hello protocol world"
 */
