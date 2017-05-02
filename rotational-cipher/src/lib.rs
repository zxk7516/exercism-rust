use std::char;

pub fn rotate(s: &str, rolate: u32) -> String{
    s.chars().map( |c| {
        match c {
            'a'...'z' => char::from_u32(((c as u32 - 97) + rolate) % 26 + 97).unwrap() ,
            'A'...'Z' => char::from_u32(((c as u32 - 65) + rolate) % 26 + 65).unwrap() ,
            _ => c,
        }
    }).collect::<String>()
}