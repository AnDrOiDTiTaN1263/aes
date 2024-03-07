use crate::{aes::*, helper::*};

mod aes;
mod constants;
mod helper;
fn main(){
    
    let test_vec: Vec<u8> = decode_hex_string("00112233445566778899aabbccddeeff");
    let key:Vec<u8> = decode_hex_string("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    let cipher_text = encrypt(test_vec, key.clone());
    println!("{:02x?}", encode_hex_string(cipher_text.clone()));
    let plain_text = decrypt(cipher_text, key);
    println!("{:02x?}", encode_hex_string(plain_text.clone()));
}


