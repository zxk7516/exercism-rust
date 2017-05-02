pub fn hamming_distance(dna1: &str, dna2: &str) -> Result<usize,&'static str> {
    if dna1.len() != dna2.len() {
        return Err("strands differ in length");
    }

    Ok(dna1.chars()
    .zip(dna2.chars())
    .map(|(x,y)| if x==y {0} else {1}).sum() )
}