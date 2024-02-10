use crate::bs58::base58_decode;
use crate::dekey::deserialize_key;
use crate::pubfrompriv::derive_public_key_from_private;

mod bs58;
mod dekey;
mod pubfrompriv;

fn main() {
    let base58_key = "tprv8ZgxMBicQKsPdisufuN1WwxfQGPpBAm9DD11kyTANuq8LDBh6nGFj1kaddVP5U9if6LypPkdnUkuxLMUFkEyMNDSreXx12hxJC6WsboYbbs";
    let bytes = base58_decode(base58_key);
    let theekey = deserialize_key(&bytes);
    let pubkey = derive_public_key_from_private(&theekey.key[1..]);
    println!("{:?}", theekey);
    println!("{:?}", pubkey);
}