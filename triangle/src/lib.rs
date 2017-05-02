pub struct Triangle{
    sides: [u32;3]
}

impl Triangle {
    pub fn build(sides: [u32;3]) -> Result<Self,()> {
        let mut v = vec![sides[0],sides[1],sides[2]];
        v.sort();
        if v[0] + v[1] <= v[2] {
            Err(())
        }else{
            Ok(
                Triangle {
                    sides: [v[0],v[1],v[2]]
            })
        }

    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[2] == self.sides[1]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[2] != self.sides[1]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[2] == self.sides[1]
    }
}