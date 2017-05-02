pub fn sum_of_multiples2(h: i32,v: &Vec<i32>) -> i32{
    (0..h).fold(0, | acc, num| {
        for vi in v{
            if num % vi ==0{
                return acc+ num;
            }
        }
        acc
    })
}

pub fn sum_of_multiples(h: u32, multiples: &Vec<u32>) -> u32 {
    (1..h).filter( |x| multiples.iter().any( |m| x%m ==0)).sum()
}