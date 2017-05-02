pub fn abbreviate(phrase: &str) -> String {
    let mut get_lower = true;
    let mut get_upper = true;
    let mut r = String::new();
    for v in phrase.chars() {
        match v {
            'a'...'z' => {
                if get_lower {r.push(v);}
                get_lower = false; get_upper = true;
            },
            'A'...'Z' => {
                if get_upper { r.push(v);}
                get_lower = false; get_upper= false;
            }
            ' ' | '-' =>{ get_lower = true; get_upper=true;}
            _ => (),
        }
    }
    r.to_uppercase()
}


pub fn abbreviate(i: &str) -> String {
    let mut acronym = String::new();
    for word in i.split(|c| c==' ' || c=='-').map(|w| w.chars().collect::<Vec<_>>()) {
        let all_upper = word.iter().filter(|&x| x.is_lowercase()).count() == 0;
        acronym.push(word[0]);
        if !all_upper {
            word.iter().skip(1).filter(|&x| x.is_uppercase()).fold(&mut acronym, |mut acc, &c| {acc.push(c); acc});
        }
    }
    acronym.to_uppercase()
}
