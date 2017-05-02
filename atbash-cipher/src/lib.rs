use std::ascii::AsciiExt;
use std::char;
pub fn encode(s1: &str) -> String {
    let code_str = "zyxwvutsrqponmlkjihgfedcba";
    s1.to_lowercase().chars().filter(|&c| {
        c.is_alphanumeric() && c.is_ascii()
    }).
    map(|c| {
        match c {
            'a'...'z' => code_str.to_string().chars().nth(c as usize -'a' as usize).unwrap(),
            _ => c ,
        }
    }).collect::<Vec<_>>()
    .chunks(5)
    .map(|c| c.iter().map(|x|*x).collect())
    .collect::<Vec<String>>()
    .join(" ")

}

pub fn decode(s: &str) -> String {
    s.chars()
        .filter(|&c| c.is_alphanumeric() && c.is_ascii())
            .map(|c| if c.is_alphabetic() {char::from_u32(122-((c as u32)-97)).unwrap()} else {c})
        .collect::<String>()
}
