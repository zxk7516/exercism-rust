pub fn is_valid(s: &str) -> bool {
    let array = s.replace(" ", "");
    if array.chars().any(|c| !c.is_digit(10)) {
        return false;
    }
    if array.len() < 2 {
        return false;
    }
    array.chars().rev()
            .map(|d| d.to_digit(10).unwrap())
            .enumerate()
            .map(|(index, d)| if index % 2 == 1 { d * 2 } else { d } )
            .map(|d| if d > 9 { d - 9 } else { d })
            .sum::<u32>() % 10 == 0
}
