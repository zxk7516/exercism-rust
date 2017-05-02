pub fn reply(q: &str) -> String {
    if  q.ends_with("?") {
        String::from("Sure.")
    }else if q.is_empty() {
        String::from("Fine. Be that way!")
    }else if q.to_uppercase() == q {
        String::from("Whoa, chill out!")
    }else {
        String::from("Whatever.")
    }
}