use crate::{aes::*, helper::*};

mod aes;
mod constants;
mod helper;
fn main(){
    
    let test_vec: Vec<u8> = decode_hex_string("00112233445566778899aabbccddeeff");
    let key:Vec<u8> = decode_hex_string("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    let text = encode_hex_string(encrypt(test_vec, key));
    if text == "8ea2b7ca516745bfeafc49904b496089".to_string(){
        println!("equal texts") ;
    }
}


