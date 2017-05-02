pub fn is_pangram(sentence: &str) -> bool{
    let every_letter = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    every_letter.chars().all(|c|{
        sentence.to_uppercase().contains(c)
    })
}