use aes::{convert_vec_to_state_array, key_expansion, mix_columns};

use crate::helper::{decode_hex_string, encode_hex_string};
#[allow(dead_code, unused)]
mod aes;
mod constants;
mod helper;
fn main(){
    
    let test_vec: Vec<u8> = decode_hex_string("60 3d eb 10 15 ca 71 be 2b 73 ae f0 85 7d 77 81
    1f 35 2c 07 3b 61 08 d7 2d 98 10 a3 09 14 df f4");
    key_expansion(test_vec);
}


