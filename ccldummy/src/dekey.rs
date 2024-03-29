#[derive(Debug)]
pub struct ExKey {
    pub(crate) version: [u8; 4],
    pub(crate) depth: [u8; 1],
    pub(crate) finger_print: [u8; 4],
    pub(crate) child_number: [u8; 4],
    pub(crate) chaincode: [u8; 32],
    pub(crate) key: [u8; 32],
}

pub fn deserialize_key(bytes: &[u8]) -> ExKey {
    let key = ExKey {
        version: bytes[0..4].try_into().unwrap(),
        depth: bytes[4..5].try_into().unwrap(),
        finger_print: bytes[5..9].try_into().unwrap(),
        child_number: bytes[9..13].try_into().unwrap(),
        chaincode: bytes[13..45].try_into().unwrap(),
        key: bytes[46..].try_into().unwrap(),
    };

    key
}

/* ExKey {
    version: [4, 53, 131, 148],
    depth: [0],
    finger_print: [0, 0, 0, 0],
    child_number: [0, 0, 0, 0],
    chaincode: [85, 90, 211, 2, 233, 209, 154, 232, 184, 15, 3, 131, 254, 37, 70, 241, 135, 236, 101, 13, 105, 123, 241, 137, 167, 129, 177, 85, 47, 102, 96, 17],
    key: [0, 19, 2, 24, 138, 40, 68, 201, 112, 206, 236, 197, 132, 119, 87, 153, 200, 211, 253, 231, 239, 41, 69, 46, 229, 239, 252, 100, 131, 1, 121, 55, 96] }

Deserialize the extended pubkey bytes and return a ExKey object
Bip32 Serialization format: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
4 byte: version bytes (mainnet: 0x0488B21E public, 0x0488ADE4 private; testnet: 0x043587CF public, 0x04358394 private)
1 byte: depth: 0x00 for master nodes, 0x01 for level-1 derived keys, ....
4 bytes: the fingerprint of the parent's key (0x00000000 if master key)
4 bytes: child number. This is ser32(i) for i in xi = xpar/i, with xi the key being serialized. (0x00000000 if master key)
32 bytes: the chain code
33 bytes: the public key or private key data (serP(K) for public keys, 0x00 || ser256(k) for private keys) */
