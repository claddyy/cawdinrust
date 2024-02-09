use crate::bs58::base58_decode;
pub mod bs58;

fn main() {
    let bytes = base58_decode("tprv8ZgxMBicQKsPdisufuN1WwxfQGPpBAm9DD11kyTANuq8LDBh6nGFj1kaddVP5U9if6LypPkdnUkuxLMUFkEyMNDSreXx12hxJC6WsboYbbs");
    println!("{:?}", bytes);
}