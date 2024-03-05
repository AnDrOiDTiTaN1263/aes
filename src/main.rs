use aes::convert_vec_to_state_array;

#[allow(dead_code, unused)]
mod aes;
mod constants;

fn main(){
    let test_vec: Vec<u8> = vec![
        0x00,0x11,0x22,0x33,
        0x44,0x55,0x66,0x77,
        0x88,0x99,0xaa,0xbb,
        0xcc,0xdd,0xee,0xff
        ];
    println!("{:02x?}",convert_vec_to_state_array(test_vec));
}


