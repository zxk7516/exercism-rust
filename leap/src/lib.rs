
/**
pub fn is_leap_year(y: u32) -> bool {
    if y % 400 == 0 {
        true
    }else if y % 100 == 0 {
        false
    }else if  y % 4  == 0 {
        true
    }else {
        false
    }
}
*/


pub fn is_leap_year(y: u32) -> bool {
    match y {
        i if i % 400 == 0 => true,
        i if i % 100 == 0 => false,
        i if i % 4 == 0 => true,
        _ => false
    }
}
