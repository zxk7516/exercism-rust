use std::cmp::PartialEq;

#[derive(Debug)]
pub struct RibonucleicAcid {
    rna: String
}

impl RibonucleicAcid {
    pub fn new(s: &str) -> Self {
        RibonucleicAcid {
            rna: String::from(s)
        }
    }
}

impl PartialEq for RibonucleicAcid {
    fn eq(&self, other: &RibonucleicAcid) -> bool {
        self.rna == other.rna
    }


    
    fn ne(&self, other: &RibonucleicAcid) -> bool { !self.eq(other) }

}

#[derive(Debug)]
pub struct DeoxyribonucleicAcid {
    dna: String
}


impl DeoxyribonucleicAcid {
    pub fn new(s: &str) -> Self {
        DeoxyribonucleicAcid {
            dna: String::from(s)
        }
    }
    pub fn to_rna(&self) -> RibonucleicAcid {
        let s = self.dna.chars().map(|c| {
            match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => '_',
            }
        }).collect::<String>();
        RibonucleicAcid{
            rna: s
        }
    }
}


