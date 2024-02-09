#[derive(Debug)]
pub struct ExKey {
    version: [u8; 4],
    depth: [u8; 1],
    finger_print: [u8; 4],
    child_number: [u8; 4],
    chaincode: [u8; 32],
    key: [u8; 33],
}

pub fn deserialize_key(bytes: &[u8]) -> ExKey {
    if bytes.len() < 78 {
        panic!("Invalid input: insufficient bytes for BIP32 key");
    }

    let version = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
    let depth = [bytes[4]];
    let finger_print = u32::from_be_bytes(bytes[5..9].try_into().unwrap());
    let child_number = u32::from_be_bytes(bytes[9..13].try_into().unwrap());

    let mut chaincode = [0u8; 32];
    chaincode.copy_from_slice(&bytes[13..45]);

    let mut key = [0u8; 33];
    key.copy_from_slice(&bytes[45..78]);

    ExKey {
        version: version.to_be_bytes(),
        depth,
        finger_print: finger_print.to_be_bytes(),
        child_number: child_number.to_be_bytes(),
        chaincode,
        key,
    }
}

/* ExKey {
    version: [4, 53, 131, 148],
    depth: [0],
    finger_print: [0, 0, 0, 0],
    child_number: [0, 0, 0, 0],
    chaincode: [85, 90, 211, 2, 233, 209, 154, 232, 184, 15, 3, 131, 254, 37, 70, 241, 135, 236, 101, 13, 105, 123, 241, 137, 167, 129, 177, 85, 47, 102, 96, 17],
    key: [0, 19, 2, 24, 138, 40, 68, 201, 112, 206, 236, 197, 132, 119, 87, 153, 200, 211, 253, 231, 239, 41, 69, 46, 229, 239, 252, 100, 131, 1, 121, 55, 96] }
 */

/* Deserialize the extended pubkey bytes and return a ExKey object
Bip32 Serialization format: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
4 byte: version bytes (mainnet: 0x0488B21E public, 0x0488ADE4 private; testnet: 0x043587CF public, 0x04358394 private)
1 byte: depth: 0x00 for master nodes, 0x01 for level-1 derived keys, ....
4 bytes: the fingerprint of the parent's key (0x00000000 if master key)
4 bytes: child number. This is ser32(i) for i in xi = xpar/i, with xi the key being serialized. (0x00000000 if master key)
32 bytes: the chain code
33 bytes: the public key or private key data (serP(K) for public keys, 0x00 || ser256(k) for private keys) */