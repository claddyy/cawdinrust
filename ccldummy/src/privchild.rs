use hmac_sha512::HMAC; 
use ripemd::Ripemd160;
use secp256k1::{PublicKey, Scalar, Secp256k1, SecretKey};
use sha2::{Sha256, Digest};

use crate::dekey::ExKey;

pub fn derive_priv_child(key: ExKey, child_num: u32) -> ExKey {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&key.key).unwrap();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let finger_print = &Ripemd160::digest(Sha256::digest(&public_key.serialize()))[0..4];

    let data = if child_num >= 0x80000000 {
        let mut data = Vec::new();
        data.push(0);
        data.extend_from_slice(&key.key);
        data.extend_from_slice(&child_num.to_be_bytes());
        data
    } else {
        let mut data = public_key.serialize().to_vec();
        data.extend_from_slice(&child_num.to_be_bytes());
        data
    };

    let hmac = HMAC::mac(&data, &key.chaincode);
    let child_key_slice = SecretKey::from_slice(&hmac[..32]).unwrap();
    let child_chaincode = &hmac[32..];
    
    let child_skey = secret_key.add_tweak(&Scalar::from(child_key_slice)).unwrap();

    
    ExKey{
        version: key.version,
        depth: [key.depth[0] + 1],
        finger_print: finger_print.try_into().unwrap(),
        child_number:child_num.to_be_bytes(),
        chaincode: child_chaincode.try_into().unwrap(),
        key: child_skey[0..32].try_into().unwrap()
    }
}
