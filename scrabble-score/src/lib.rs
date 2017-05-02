pub fn score2(s: &str) -> u32 {
    let ss = s.to_uppercase();
    let mut r = 0;
    for c in ss.chars() {
        r += match c {
            'A' |'E' |'I' |'O' |'U' |'L' |'N' |'R' |'S' | 'T'  => 1,
            'D'| 'G'=> 2,
            'B'|'C'|'M'|'P' => 3,
            'F'| 'H'| 'V'| 'W'| 'Y'=>4 ,
            'K' =>5,
            'J'| 'X' => 8,
            'Q'| 'Z' => 10,
            _ => 0
        }
    }
    r
}


fn char2score(c: char) -> u32 {
    match c {
            'A' |'E' |'I' |'O' |'U' |'L' |'N' |'R' |'S' | 'T'  => 1,
            'D'| 'G'=> 2,
            'B'|'C'|'M'|'P' => 3,
            'F'| 'H'| 'V'| 'W'| 'Y'=>4 ,
            'K' =>5,
            'J'| 'X' => 8,
            'Q'| 'Z' => 10,
            _ => 0
    }
}

pub fn score(word: &str) -> u32 {
    word.to_uppercase().chars().map(char2score).sum()
}
