pub fn decode_hex_string(s: &str) -> Vec<u8> {
    let mut ret:Vec<u8> = vec![];
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
            string += &format!("{:02x}",input[c][r]);
        }
    }
    string
}   