pub struct PascalsTriangle{
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            PascalsTriangle{
                rows: Vec::new()
            }
        }else{
            let mut rows: Vec<Vec<u32>> = Vec::new();
            let row_first: Vec<u32> = vec![1];
            rows.push(row_first);
            for x in 2..(row_count + 1) {
                let mut row: Vec<u32> = Vec::new();
                row.push(1);

                for i in 1..(x-1) {
                    row.push( rows[rows.len()-1][(i-1) as usize] +  rows[rows.len()-1][(i) as usize])
                }

                row.push(1);
                rows.push(row)
            }
            PascalsTriangle{
                rows: rows
            }
        }
        
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
