pub fn encode(data: &str) -> String{
    let mut encoded_data = String::new();
    let mut char_iter = data.chars();

    if data.is_empty() {
        return encoded_data;
    }

    let mut current_count = 1;
    let mut current_char = char_iter.next().unwrap().clone();

    for c in char_iter {
        if c != current_char {
            match current_count {
                1 => encoded_data.push(current_char),
                _ => encoded_data.push_str(&format!("{}{}", current_count, current_char)),
            };
            current_char = c;
            current_count = 1;
        } else {
            current_count += 1;
        }
    }
    match current_count {
        1 => encoded_data.push(current_char),
        _ => encoded_data.push_str(&format!("{}{}", current_count, current_char)),
    };
    encoded_data
}

pub fn decode(encoded_data: &str) -> String{
    let mut number_buf = String::new();
    let mut output_buf = String::new();
    for c in encoded_data.chars() {
        if c.is_digit(10) {
            number_buf.push(c)
        } else {
            if number_buf.is_empty() {
                output_buf.push(c);
            } else {
                let repeat: usize = number_buf.parse().unwrap();
                output_buf.push_str(&c.to_string().repeat(repeat));
                number_buf.clear();
            }
        }
    }
    output_buf

}