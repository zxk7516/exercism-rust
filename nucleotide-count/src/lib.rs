use std::collections::HashMap;

pub fn count(c: char, dna: &str) -> Result<i32, ()> {
    let dna_syms = "ACGT";
    if !dna_syms.contains(c){
        return Err(());
    }
    let mut count = 0;
    for m in dna.chars() {
        match m {
            i if i == c => count += 1,
            'A'|'C'|'G'|'T' => count += 0,
            _ => return Err(())
        }
    }
    Ok(count)
} 

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, ()> {
    let mut h = HashMap::new();
    h.insert('A',0);
    h.insert('C',0);
    h.insert('G',0);
    h.insert('T',0);
    for m in dna.chars() {
        if !"ACGT".contains(m) {
            return Err(());
        }
        match h.get(&m){
            Some(&count) => h.insert(m, count + 1),
            _ => h.insert(m, 1)
        };
    }
    Ok(h)
}
