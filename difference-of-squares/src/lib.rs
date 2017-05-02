pub fn difference2(h: u64) -> u64 {
    let mut r:u64 = 0;
    let h1  = h + 1;
    for x in 1..h1 {
        for a in (x+1)..h1 {
            r += 2 * x * a;
        }
    }
    r
}

pub fn square_of_sum2(h: u64) -> u64 {
    let mut sum = 0;
    for x in 1..(h+1) {
        sum += x;
    }
    sum * sum
}


pub fn sum_of_squares2(h: u64) -> u64 {
    let mut sum = 0;
    for x in 1..(h+1) {
        sum += x*x;
    }
    sum
}


pub fn square_of_sum(num: u64) -> u64 {
    (1..num+1).fold(0,| acc, num| acc + num).pow(2)
}

pub fn sum_of_squares(num: u64) -> u64 {
    (1..num+1).fold(0, |acc, num| acc + num.pow(2))
}

pub fn difference(num: u64) -> u64{
    square_of_sum(num) - sum_of_squares(num)
}

