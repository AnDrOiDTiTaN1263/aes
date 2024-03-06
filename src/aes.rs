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

use crate::{constants::*, helper::{calc_mix_col_val, calc_rcon, left_shift_bytes, xor_vec}};

///single byte substitution by using the lookup table called S-BOX in constants.rs
pub fn s_byte(byte:u8)->u8{
    S_BOX[byte as usize]
}

pub fn vec_sub_bytes(mut input_vec:Vec<u8>)->Vec<u8>{
    for x in 0..input_vec.len(){
        input_vec[x] = s_byte(input_vec[x]);
    }
    input_vec
}   

pub fn sub_bytes(mut state_array:Vec<Vec<u8>>)->Vec<Vec<u8>>{
    for r in 0..4{
        for c in 0..4{
            state_array[r][c] = s_byte(state_array[r][c]);
        }
    }
    state_array
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
    for r in 0..4{
        for c in 0..4{
            temp[r][c] = state_array[r][((c+shift(r)).rem_euclid(4))];
        }
    }
    temp
}

pub fn add_round_key(mut state_array:Vec<Vec<u8>>, keys:Vec<Vec<u8>>)->Vec<Vec<u8>>{
    /*
    [5f, 57, f7, 1d]
    [72, f5, be, b9]
    [64, bc, 3b, f9]
    [15, 92, 29, 1a]
 */
    for c in 0..4{
        let col = [state_array[0][c], state_array[1][c], state_array[2][c], state_array[3][c]];
        let vals = xor_vec(col.to_vec(), keys[c].clone());
        for x in 0..4{
            state_array[x][c] = vals[x];
        }
        let col = [state_array[0][c], state_array[1][c], state_array[2][c], state_array[3][c]];
    }
    state_array
}


pub fn key_expansion(input_key:Vec<u8>)->Vec<Vec<u8>>{
    /*
        With AES256 we require 14 + 1 round keys, 
     */
    let mut i = 0;
    let mut ret:Vec<Vec<u8>> = vec![vec![0u8;4];60];
    while i <NK{
        ret[i] = input_key[i*4..i*4+4].to_vec();
        i+=1;
    }
    i = NK;
    while i< NB * (NR+1){
        let mut temp = ret[i-1].to_vec();
        if i.rem_euclid(NK) == 0{
            temp = xor_vec(
                    vec_sub_bytes(left_shift_bytes(temp, 1)), calc_rcon(i/NK).to_vec());
        }
        else if NK > 6 && i.rem_euclid(NK) == 4{
            temp = vec_sub_bytes(temp);
        }
        ret[i] = xor_vec(ret[i-NK].clone(), temp.clone());
        i+=1;
    }
    ret

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
