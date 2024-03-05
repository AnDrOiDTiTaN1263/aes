/*
    most code derived from official NIST publications:
    https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf
    and it's updated version
    https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197-upd1.pdf
*/

/*
    functionality to be implemented:
        key expansion
            -round constants
        add round key
        convert input vector to state array

        encrypt mode:
            sub bytes
                -rijndael substitution box
            shift rows
            mix columns
        decrypt mode:
            inverse sub bytes
            inverse shift rows
            inverse mix columns
*/

use crate::constants::S_BOX;

///single byte substitution by using the lookup table called S-BOX in constants.rs
pub fn sub_byte(byte:u8)->u8{
    S_BOX[byte as usize]
}

pub fn vec_sub_bytes(mut input_vec:Vec<u8>){
    for x in 0..input_vec.len(){
        input_vec[x] = sub_byte(input_vec[x]);
    }
}   


pub fn convert_vec_to_state_array(input_vec:Vec<u8>)->Vec<Vec<u8>>{
    if input_vec.len() != 16{
        return vec![];
    }
    let mut state_array:Vec<Vec<u8>> = vec![vec![0u8;4];4];
    for c in 0..4{
        for r in 0..4{
            state_array[c][r] = input_vec[r+4*c];
        }
    }
    state_array

}

fn key_expansion(){

}