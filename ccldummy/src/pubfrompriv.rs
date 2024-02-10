use secp256k1::{Secp256k1, SecretKey, PublicKey};

pub fn derive_public_key_from_private(key: &[u8]) -> Vec<u8> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(key).expect("Invalid Private Key");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let compressed_public_key = public_key.serialize();

    compressed_public_key.to_vec()
}

/* Setup a context struct/object for our secp256k1 curve.
and obtain our compressed public key. key should be of 32
bytes and the extra should be truncated.

Expected output:
[3, 170, 48, 9, 75, 235, 238, 102, 215, 143, 150, 203, 83, 51, 8, 14, 99, 89, 186, 76, 64, 48, 152, 39, 242, 128, 202, 34, 140, 248, 53, 192, 124]
*/