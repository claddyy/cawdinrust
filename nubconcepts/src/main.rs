// Use only the secp256k1 crate
use secp256k1::{PublicKey, Secp256k1, SecretKey};

// ... rest of the code remains the same ...

fn derive_public_key_from_private(key: &[u8]) -> Vec<u8> {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(key).expect("Invalid private key");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let compressed_public_key = public_key.serialize_uncompressed();

    compressed_public_key.to_vec()
}

fn main() {
    // Generate a random 32-byte private key
    let mut private_key = [0u8; 32];
    getrandom::getrandom(&mut private_key).expect("Failed to generate random bytes");

    // Derive the compressed public key
    let public_key = derive_public_key_from_private(&private_key);

    // Print the keys in hexadecimal format
    println!("Private key: {:x?}", private_key);
    println!("Compressed public key: {:x?}", public_key);
}
