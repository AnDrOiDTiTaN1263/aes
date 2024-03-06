use crate::constants::{G_MUL_2, G_MUL_3, MIX_COL_MATRIX, RCON_VALUES};

pub fn decode_hex_string(s: &str) -> Vec<u8> {
    let mut ret:Vec<u8> = vec![];
    let s:String = s.chars().filter(|c| !c.is_whitespace()).collect();
    if s.len() % 2 != 0{
        return ret;
    }
    for x in (0..s.len()).step_by(2){
        ret.push(u8::from_str_radix(&s[x..x+2], 16).unwrap());
    }
    ret
}

//used to display a state array as a long string of characters, NIST publication does it this way
//this is to make it easier for us to follow along with them and test the code as we go
pub fn encode_hex_string(input:Vec<Vec<u8>>)->String{
    let mut string = "".to_string();
    for c in 0..input.len(){
        for r in 0..input[c].len(){
            string += &format!("{:02x}",input[r][c]);
        }
    }
    string
}   

pub fn calc_mix_col_val(col:[u8;4], matrix_col_index:usize)->Option<u8>{
    if matrix_col_index >3{
        println!("matrix column index given to calc mix columns was incorrect, must be between 0 and 4");
        return None;
    }
    let mix_col_multipliers = &MIX_COL_MATRIX[matrix_col_index*4..matrix_col_index*4+4];
    let mut ret = 0u8;
    for x in 0..4{
        if mix_col_multipliers[x] == 1{
            ret ^= col[x];
        }
        if mix_col_multipliers[x] == 2{
            ret ^= G_MUL_2[col[x] as usize]
        }
        if mix_col_multipliers[x] == 3{
            ret ^= G_MUL_3[col[x] as usize]
        }
    }

    Some(ret)
}
#[allow(unused)]
pub fn xor_vec(left:Vec<u8>, right:Vec<u8>)->Vec<u8>{
    let mut ret: Vec<u8> = vec![];
    let mut right = right.clone();
    let mut left = left.clone();
    if left.len() < right.len(){
        //push the extra bits as they are 
        for _ in 0..right.len()-left.len(){
            ret.push(right.remove(0));
        }
    }if right.len() < left.len(){
        for _ in 0..left.len()-right.len(){
            ret.push(left.remove(0));
        }
        
    }
    for x in 0..left.len(){
        ret.push(left[x]^right[x]);
    }
    ret
}

pub fn calc_rcon(index:usize)->[u8;4]{
    [RCON_VALUES[index],0u8,0u8,0u8]
}

pub fn left_shift_bytes(input_vec:Vec<u8>, times:usize)->Vec<u8>{
    if times >= input_vec.len(){
        println!("didn't shift anything because \'times\' was larger than the length of the input vector");
        return input_vec;
    }
    let right_seg = input_vec[times..input_vec.len()].to_vec();
    let mut ret = right_seg;
    ret.append(&mut input_vec[0..times].to_vec());
    ret
}