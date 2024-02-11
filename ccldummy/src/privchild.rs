use bitcoin::hashes::Hmac;
use sha2::Sha512;
use crate::dekey::ExKey;

type HmacSha512 = Hmac<Sha512>;
pub fn derive_priv_child(key: ExKey, child_num: u32) -> ExKey {
    let child_path  = "m/84h/1h/0h/0/*";
    let(kpar, cpar) = (&key.key, &key.chaincode);
    let is_hardened = child_num >= 0x80000000;
    let mut data =  Vec::new();

    if is_hardened{
        data.push(0x00);
    }
    data.extend_from_slice(&kpar);
    data.extend_from_slice(&child_num.to_be_bytes());

    let mut hmac = HmacSha512::new_from_slice(&cpar).unwrap();
    hmac.update(&data);
    let result  = hmac.finalize().into_bytes();

    let il = &result[0..32];
    let ir = &result[32..64];

    let mut ki_bytes  = [0u8; 32];
    ki_bytes.copy_from_slice(il);
    let mut kpar_bytes = [0u8; 32];
    kpar_bytes[1..].copy_from_slice(&kpar[1..]);

    // Compute ki = IL + kpar (mod n)
    let mut ki = [0u8; 32];
    let mut carry = 0;
    for i in (0..32).rev() {
        let sum = il[i] + kpar_bytes[i] + carry;
        ki[i] = sum % 256;
        carry = sum / 256;
    }

    let curve_order = &secp256k1::constants::CURVE_ORDER;
    if ki >= *curve_order {
        panic!("Invalid child key: ki is greater than or equal to the curve order");
    }

    // Build the child ExKey
    let mut child_key = ExKey {
        version: key.version,
        depth: [key.depth[0] + 1], // Increment the depth
        finger_print: key.finger_print,
        child_number: child_num.to_be_bytes(),
        chaincode: ir.try_into().unwrap(),
        key: [0; 33], // Placeholder for the key
    };

    // Set the child key
    child_key.key[1..].copy_from_slice(&ki);

    child_key
}