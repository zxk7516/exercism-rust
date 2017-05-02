pub fn lsp(number: &str, l: usize) -> Result<u32,()>{
    if l == 0 {
        return Ok(1);
    }
    if number.len() + 1 - l <= 0 {
        return Err(())
    }

    let mut numbers: Vec<u32>   = Vec::new();
    for c in number.chars() {
        match c.to_digit(10){
            Some(n) => numbers.push(n),
            None => return Err(()),
        }
    }

    Ok(numbers.windows(l).map::<u32, _>(
        |x| x.iter().product()  
    ).max().unwrap())
}