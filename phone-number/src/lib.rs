pub fn number(num: &str) -> Option<String> {
    let mut s = num.chars().filter(|&c|{
        c.is_numeric()
    }).collect::<String>();
    match s.len(){
        10 => Some(s),
        11 => if s.remove(0) == '1' {
                Some(s)
            }else{
                None
            },
        _ => None
    }
}


pub fn area_code(s: &str) -> Option<String> {
    let n = number(s);
    match n {
        Some(x) => Some(x[0..3].to_string() ),
        None => None,
    }
}

pub fn pretty_print(s: &str) ->String {
   let n = number( s );
   match n {
      Some(x) => format!( "({}) {}-{}", &x[0..3], &x[3..6], &x[6..10] ),
      None => "invalid".to_string(),
   }

}