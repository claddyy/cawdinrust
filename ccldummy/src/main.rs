use crate::bs58::base58_decode;
use crate::dekey::deserialize_key;

mod bs58;
mod dekey;

fn main() {
    let bytes = base58_decode("tprv8ZgxMBicQKsPdisufuN1WwxfQGPpBAm9DD11kyTANuq8LDBh6nGFj1kaddVP5U9if6LypPkdnUkuxLMUFkEyMNDSreXx12hxJC6WsboYbbs");
    let theekey = deserialize_key(&bytes);
    println!("{:?}", bytes);
    println!("{:?}", theekey);
}