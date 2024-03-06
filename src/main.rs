use aes::{add_round_key, convert_vec_to_state_array, key_expansion, mix_columns};

use crate::{aes::{convert_state_array_to_vec, shift_rows, sub_bytes, vec_sub_bytes}, helper::{decode_hex_string, encode_hex_string}};
#[allow(dead_code, unused)]
mod aes;
mod constants;
mod helper;
fn main(){
    
    let test_vec: Vec<u8> = decode_hex_string("00112233445566778899aabbccddeeff");
    // println!("testv: {test_vec:02x?}");
    let key:Vec<u8> = decode_hex_string("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    let keys = key_expansion(key);
    let mut state_array = convert_vec_to_state_array(test_vec);
    // println!("state: {:02x?}", state_array);
    // println!("key_v:{:02x?}",keys[0..4].to_vec());
    state_array = add_round_key(state_array.clone(), keys[0..4].to_vec());
    println!("add_r: {:02x?}",state_array);
    state_array = sub_bytes(state_array.clone());
    println!("sub_b: {:02x?}",state_array);
    state_array = shift_rows(state_array);
    println!("sht_r: {:02x?}",state_array);
    state_array = mix_columns(state_array);
    println!("mix_c: {:02x?}",state_array);
    let fin_vec = convert_state_array_to_vec(state_array);
    println!("fin_v:{:02x?}",fin_vec);
}


