pub fn square(s: u32) -> u64 {
    if s > 64 || s == 0 {
        panic!("Square must be between 1 and 64");
    }
    1 << (s-1)
}

pub fn total() -> u64 {
    (1..65).fold(0, |acc, num|{ acc + square(num) } )
}
pub fn total2() -> u64 {
    (1..65).map(square).sum()
}
