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

use crate::{constants::*, helper::calc_mix_col_val};

///single byte substitution by using the lookup table called S-BOX in constants.rs
pub fn sub_byte(byte:u8)->u8{
    S_BOX[byte as usize]
}

pub fn vec_sub_bytes(mut input_vec:Vec<u8>){
    for x in 0..input_vec.len(){
        input_vec[x] = sub_byte(input_vec[x]);
    }
}   

fn shift(r:usize)->usize{
    if r == 1{
        return 1;
    }if r == 2{
        return 2;
    }if r == 3{
        return 3;
    }
    0
}

pub fn shift_rows(state_array:Vec<Vec<u8>>)->Vec<Vec<u8>>{
    let mut temp = vec![vec![0u8;4];4];
    println!("{state_array:02x?}");
    for r in 0..4{
        for c in 0..4{
            temp[r][c] = state_array[r][((c+shift(r)).rem_euclid(4))];
        }
    }
    temp
}


pub fn convert_vec_to_state_array(input_vec:Vec<u8>)->Vec<Vec<u8>>{
    if input_vec.len() != 16{
        return vec![];
    }
    let mut state_array:Vec<Vec<u8>> = vec![vec![0u8;4];4];
    for c in 0..4{
        for r in 0..4{
            state_array[r][c] = input_vec[r+4*c];
        }
    }
    state_array

}

fn key_expansion(){

}
pub fn mix_columns(mut state_array:Vec<Vec<u8>>)->Vec<Vec<u8>>{
    /*
        mix columns matrix:
        [02,03,01,01]
        [01,02,03,01]
        [01,01,02,03]
        [03,01,01,02]
        s'[0,c] = (02*s[0,c]) XOR (03*s[1,c]) XOR s[2,c] XOR s[3,c]
        s'[1,c] = s[0,c] XOR (02*s[1,c]) XOR (03*s[2,c]) XOR s[3,c]
        s'[2,c] = s[0,c] XOR s[1,c] XOR (02*s[2,c]) XOR (03*s[3,c])
        s'[3,c] = (03*s[0,c]) XOR s[1,c] XOR s[2,c] XOR (02*s[3,c])
     */
    for c in 0..4{
        let col = [state_array[0][c],state_array[1][c],state_array[2][c],state_array[3][c]];
        state_array[0][c] = calc_mix_col_val(col.clone(), 0).unwrap();
        state_array[1][c] = calc_mix_col_val(col.clone(), 1).unwrap();
        state_array[2][c] = calc_mix_col_val(col.clone(), 2).unwrap();
        state_array[3][c] = calc_mix_col_val(col.clone(), 3).unwrap();
    }
    state_array
}