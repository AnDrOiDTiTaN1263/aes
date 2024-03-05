use aes::{convert_vec_to_state_array, shift_rows};

use crate::helper::{decode_hex_string, encode_hex_string};
#[allow(dead_code, unused)]
mod aes;
mod constants;
mod helper;
fn main(){
    
    let test_vec: Vec<u8> = decode_hex_string("63cab7040953d051cd60e0e7ba70e18c");
    // println!("{:02x?}",test_vec);
    let shifted = shift_rows(convert_vec_to_state_array(test_vec));
    println!("{:02x?}",shifted);
    //63 ca b7 04 
    //09 53 d0 51
    //cd 60 e0 e7
    //ba 70 e1 8c
    
    //63 53 e0 8c 
    //09 60 e1 04
    //cd 70 b7 51
    //ba ca d0 e7
    
}


