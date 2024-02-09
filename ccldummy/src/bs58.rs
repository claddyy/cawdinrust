use bs58::decode;

pub fn base58_decode(base58_string: &str) -> Vec<u8> {
    let mut decoded_bytes = decode(base58_string).into_vec().expect("Invalid base58 string");
    decoded_bytes.truncate(decoded_bytes.len() - 4);
    decoded_bytes
    // BONUS points for verifying checksum
}
/*output: [4, 53, 131, 148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 90,
211, 2, 233, 209, 154, 232, 184, 15, 3, 131, 254, 37, 70, 241,
135, 236, 101, 13, 105, 123, 241, 137, 167, 129, 177, 85, 47,
102, 96, 17, 0, 19, 2, 24, 138, 40, 68, 201, 112, 206, 236,
197, 132, 119, 87, 153, 200, 211, 253, 231, 239, 41, 69, 46,
229, 239, 252, 100, 131, 1, 121, 55, 96]

we represent bytes in rust, using data types like u8. u8[0,255]
1 byte = 8 bits, and u8 with 256 possible values covers all the
possible combinations of 8 bits.

So, for eg. 25 represents the byte value of 0110 0101
*/