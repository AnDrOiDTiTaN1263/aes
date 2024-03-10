use crate::constants::*;

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
pub fn encode_hex_string(input:Vec<u8>)->String{
    let mut string = "".to_string();
    for c in 0..input.len(){
        string += &format!("{:02x?}", input[c]);
    }
    string
}   

pub fn calc_mix_col_val(col:[u8;4], matrix_col_index:usize)->Option<u8>{
    if matrix_col_index >3{
        println!("matrix column index given to calc mix columns was incorrect, must be between 0 and 3");
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
pub fn inv_calc_mix_col_val(col:[u8;4], matrix_col_index:usize)->Option<u8>{
    if matrix_col_index >3{
        println!("matrix column given to inv calc mix columns was incorrect, must be between 0 and 3");
        return None;
    }
    let mut ret= 0u8;
    let mix_col_multipliers = &INV_MIX_COL_MATRIX[matrix_col_index*4..matrix_col_index*4+4];
    for x in 0..4{
        if mix_col_multipliers[x] == 14{
            ret ^= G_MUL_14_TABLE[col[x] as usize];
        }if mix_col_multipliers[x] == 13{
            ret ^= G_MUL_13_TABLE[col[x] as usize];
        }if mix_col_multipliers[x] == 11{
            ret ^= G_MUL_11_TABLE[col[x] as usize];
        }if mix_col_multipliers[x] == 9{
            ret ^= G_MUL_9_TABLE[col[x] as usize];
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

pub fn convert_state_array_to_vec(state_array:Vec<Vec<u8>>)->Vec<u8>{
    let mut ret: Vec<u8> = vec![0u8;16];
    for r in 0..4{
        for c in 0..4{
            ret[r+4*c] = state_array[r][c];
        }
    }
    ret
}

pub fn galois_multiplication(left:u8, right:u8)->u8{
    let mut left = left.clone() as u16;
    let mut right = right.clone() as u16;
    let mut ret = 0u16;
    while left != 0 && right != 0{
        if right&1 == 1{
            ret ^=left;
        }
        if (left & 0x80) == 1{
            left = (left<<1)^0x11b;
        }else{
            left <<=1;
        }
        right >>=1;
    }
    ret as u8
}