pub fn primes_up_to(upper: u32) -> Vec<u32> {
    if upper <= 1{
        Vec::new()
    } else if  upper == 2{
        vec![2]
    } else {
        let mut v: Vec<u32> = vec![2,3];
        let mut max: u32 = 3;
        while max <= upper {
            if v.iter().any(|x| {max % x == 0}) {
                max = max + 2;                
                continue;
            }else{
                v.push(max);
            }
        }
        v
    }

    
}