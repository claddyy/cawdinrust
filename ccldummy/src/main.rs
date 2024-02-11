use crate::bs58::base58_decode;
use crate::dekey::deserialize_key;
use crate::privchild::derive_priv_child;

mod bs58;
mod dekey;
mod pubfrompriv;
mod privchild;

fn main() {
    let base58_key = "tprv8ZgxMBicQKsPdisufuN1WwxfQGPpBAm9DD11kyTANuq8LDBh6nGFj1kaddVP5U9if6LypPkdnUkuxLMUFkEyMNDSreXx12hxJC6WsboYbbs";
    let bytes = base58_decode(base58_key);
    let theekey = deserialize_key(&bytes);

    let child_num: u32 = 1;

    let child_key = derive_priv_child(theekey, child_num);

    println!("Child Private Key: {:x?}", child_key.key[1..]);
    println!("Chain Code: {:x?}", child_key.chaincode);
}
