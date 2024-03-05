use aes::{convert_vec_to_state_array, mix_columns};

use crate::helper::{decode_hex_string, encode_hex_string};
#[allow(dead_code, unused)]
mod aes;
mod constants;
mod helper;
fn main(){
    
    let test_vec: Vec<u8> = decode_hex_string("84e1fd6b1a5c946fdf4938977cfbac23");
    // println!("{:02x?}",test_vec);
    let state_array = convert_vec_to_state_array(test_vec);
    let mix_col_str = encode_hex_string(mix_columns(state_array));
    assert_eq!(mix_col_str, "bd2a395d2b6ac438d192443e615da195".to_string());    
}


