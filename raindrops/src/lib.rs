pub fn raindrops(i: u32) ->  String {
    let mut a : String  = String::from("");
    let mut s: bool = true;
    if i % 3 == 0 {
        a.push_str("Pling");
        s = false;
    }
    if i % 5 == 0 {
        a.push_str("Plang");
        s = false;
    }
    if i % 7 == 0 {
        a.push_str("Plong");
        s = false;
    }
    if s{
        a = format!("{}", i);
    }
    a

}