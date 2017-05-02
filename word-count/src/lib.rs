use std::collections::HashMap;

pub fn word_count2(sentense: &str) -> HashMap<String, u32> {
    let mut m = HashMap::new();
    let v: Vec<&str> = sentense. split(' ').collect();
    for w in v {
        match m.get(w){
            Some(&c) => m.insert(String::from(w), c+1),
            _ => m.insert(String::from(w), 1),
        };
    }
    m
}

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut m = HashMap::new();
    for word in s.to_lowercase().chars().filter(|&c| !":!&@$%^,;".contains(c))
        .collect::<String>().split(" ")
        .filter(|w| w.len() > 0 ){
            let counter = m.entry(word.to_string()).or_insert(0);
            *counter += 1
    }
    m
}

