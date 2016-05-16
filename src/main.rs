use std::io;
use std::io::{ BufWriter};
use std::io::prelude::*;
use std::char;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read count");
    line = line[0..line.len()-1].to_string();
    let count = u32::from_str_radix(&line.as_str(), 10).expect("failed to convert");
    let mut index = 0;
    let inp = io::stdin();
    let mut out = BufWriter::new(io::stdout());
    for curline in inp.lock().lines() {
        let next_palindrome = get_next_palindrome( &curline.unwrap());
        out.write_all( next_palindrome.as_str().as_bytes()).unwrap();
        out.write_all( "\n".as_bytes()).unwrap();
        if index % 50 == 0 { out.flush().unwrap(); }
        index += 1;
        if index == count { break; }
    }
    out.flush().unwrap();
}

fn get_next_palindrome(line: &str) -> String {
    let ret: String;
    let length = line.len();
    if length < 2 || (length == 2 && line == "10") { ret = "11".to_string(); }
    else {
        let half:usize = length >> 1; 
        
        let center_len = length - half * 2;
        let (left, center_right) = line.split_at(half);
        let left_len = left.len();
        let rleft = reverse(left);
        let right = if center_len == 0 { center_right}
                else { 
                    let (_, trueright) = center_right.split_at(1);
                    trueright
                };
        let center = if center_len == 0 { "" }
                else { 
                    let (ret, _) = center_right.split_at(1);
                    ret
                };
        if right < rleft.as_str() { 
            ret = left.to_string() + center + rleft.as_str(); 
        } else { 
            let next_left_center = inc_string( (left.to_string() + center).as_str());
            let next_left_center_len = next_left_center.len();
            let (next_left, next_center) = if next_left_center_len == (left_len + center_len) {
                        next_left_center.split_at( left_len)
                    } else {
                        if center_len == 0 { next_left_center.split_at(next_left_center_len - 1)}
                        else { 
                            let (newnext_left,_) = next_left_center.split_at(next_left_center_len - 1);
                            (newnext_left, "") 
                        }
                    };
            let rnext_left = reverse(next_left);
            ret = next_left.to_string() + next_center + rnext_left.as_str();
        }

    }
    ret
}


fn inc_string(line: &str) -> String {
    let mut rline = String::new();
    let mut increasing = true;
    
    for c in line.chars().rev() {
        if increasing {
            if c == '9' 
            {
                rline.push('0');
            }else
            {
                let newdigit: u32 = c.to_digit(10).expect("impossible") + 1;
                let mnewchar: Option<char> = char::from_digit(newdigit, 10);
                match mnewchar {
                    Some(value) => { rline.push(value); }
                    _ => {} 
                }
                increasing = false;
            }
        }else{
            rline.push(c);
        }
    }
    if increasing { rline.push('1');}
    reverse(rline.as_str())
}


fn reverse(line: &str) -> String {
    let mut ret: String = String::new();
    for c in line.chars().rev() {
        ret.push(c);
    }
    ret
}
