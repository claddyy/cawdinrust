// use std::convert::TryInto;
// use bs58;

// struct ExKey {
//     // Define your ExKey structure fields here
//     // For example:
//     version: u32,
//     depth: u8,
//     parent_fingerprint: u32,
//     child_number: u32,
//     chain_code: [u8; 32],
//     public_key: [u8; 33],
// }

// fn deserialize_key(bytes: &[u8]) -> ExKey {
//     // Check if the input bytes are at least 78 bytes long (the minimum required length for a BIP32 key)
//     if bytes.len() < 78 {
//         panic!("Invalid input: insufficient bytes for BIP32 key");
//     }

//     // Read the version bytes (4 bytes)
//     let version = u32::from_be_bytes(bytes[0..4].try_into().unwrap());

//     // Read the depth (1 byte)
//     let depth = bytes[4];

//     // Read the parent's key fingerprint (4 bytes)
//     let parent_fingerprint = u32::from_be_bytes(bytes[5..9].try_into().unwrap());

//     // Read the child number (4 bytes)
//     let child_number = u32::from_be_bytes(bytes[9..13].try_into().unwrap());

//     // Read the chain code (32 bytes)
//     let mut chain_code = [0u8; 32];
//     chain_code.copy_from_slice(&bytes[13..45]);

//     // Read the public key data (33 bytes)
//     let mut public_key = [0u8; 33];
//     public_key.copy_from_slice(&bytes[45..78]);

//     ExKey {
//         version,
//         depth,
//         parent_fingerprint,
//         child_number,
//         chain_code,
//         public_key,
//     }
// }

// fn base58_decode(base58_string: &str) -> Vec<u8> {
//     let mut decoded_bytes = bs58::decode(base58_string).into_vec().expect("Invalid base58 string");
//     decoded_bytes.truncate(decoded_bytes.len() - 4);
//     decoded_bytes
// }

// fn main() {
//     // Example usage:
//     let xpub = "xpub6Cx5tvq6nACSLJdra1A6WjqTo1SgeUZRFqsX5ysEtVBMwhCCRa4kfgFqaT2o1kwL3esB1PsYr3CUdfRZYfLHJunNWUABKftK2NjHUtzDms2";

//     // Decode the base58 string into bytes
//     let decoded_bytes = base58_decode(xpub);

//     // Deserialize the BIP32 extended public key
//     let ex_key = deserialize_key(&decoded_bytes);

//     // Now you can use the ex_key object as needed
//     println!("Version: {}", ex_key.version);
//     println!("Depth: {}", ex_key.depth);
//     println!("Parent Fingerprint: {:x}", ex_key.parent_fingerprint);
//     println!("Child Number: {}", ex_key.child_number);
//     println!("Chain Code: {:x?}", ex_key.chain_code);
//     println!("Public Key: {:x?}", ex_key.public_key);
// }

// fn deserialize_key(bytes: &[u8]) -> ExKey {
//     // Check if the input bytes are at least 78 bytes long (the minimum required length for a BIP32 key)
//     if bytes.len() < 78 {
//         panic!("Invalid input: insufficient bytes for BIP32 key");
//     }

//     let version = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
//     let depth = [bytes[4]];
//     let finger_print = u32::from_be_bytes(bytes[5..9].try_into().unwrap());
//     let child_number = u32::from_be_bytes(bytes[9..13].try_into().unwrap());

//     let mut chaincode = [0u8; 32];
//     chaincode.copy_from_slice(&bytes[13..45]);

//     let mut key = [0u8; 33];
//     key.copy_from_slice(&bytes[45..78]);

//     ExKey {
//         version: version.to_be_bytes(),
//         depth,
//         finger_print: finger_print.to_be_bytes(),
//         child_number: child_number.to_be_bytes(),
//         chaincode,
//         key,
//     }
// }


// fn base58_decode(base58_string: &str) -> Vec<u8> {
//     let mut decoded_bytes = bs58::decode(base58_string).into_vec().expect("Invalid base58 string");
//     decoded_bytes.truncate(decoded_bytes.len() - 4);
//     decoded_bytes
// }

// fn main() {
//     // Example usage:
//     let xpub = "xpub6Cx5tvq6nACSLJdra1A6WjqTo1SgeUZRFqsX5ysEtVBMwhCCRa4kfgFqaT2o1kwL3esB1PsYr3CUdfRZYfLHJunNWUABKftK2NjHUtzDms2";

//     // Decode the base58 string into bytes
//     let decoded_bytes = base58_decode(xpub);

//     // Deserialize the BIP32 extended public key
//     let ex_key = deserialize_key(&decoded_bytes);

//     // Now you can use the ex_key object as needed
//     println!("Version: {:x?}", ex_key.version);
//     println!("Depth: {:x?}", ex_key.depth);
//     println!("Parent Fingerprint: {:x?}", ex_key.finger_print);
//     println!("Child Number: {:x?}", ex_key.child_number);
//     println!("Chain Code: {:x?}", ex_key.chaincode);
//     println!("Public Key: {:x?}", ex_key.key);
// }