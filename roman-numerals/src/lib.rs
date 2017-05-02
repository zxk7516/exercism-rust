use std::string::ToString;

pub struct Roman{
    s: String
}

impl Roman {
    pub fn from(n: u32) -> Self{
        let mut s = String::new();
        let m = n / 1000;
        let c = (n % 1000) / 100;
        let x = (n % 100) / 10;
        let i = n % 10;
        
        s.push_str( &("M".repeat(m as usize)) );
        if c <= 3 {
            s.push_str( &("C".repeat(c as usize)) );
        } else if c < 5{
            s.push_str("CD");
        }else if c < 9{
            s.push_str("D");
            s.push_str(  &("C".repeat(c as usize -5)) )
        }else {
            s.push_str( "CM")
        }


        if x <= 3 {
            s.push_str( &("X".repeat(x as usize)) );
        } else if x < 5 {
            s.push_str("XL");
        }else if x < 9 {
            s.push_str( "L" );
            s.push_str( &("X".repeat( x as usize - 5)) );
        }else {
            s.push_str( "XC")
        }

        if i <= 3 {
            s.push_str( &("I".repeat(i as usize)) );
        }else if i < 5{
            s.push_str("IV");
        }else if i < 9{
            s.push_str( "V" );
            s.push_str( &("I".repeat( i as usize - 5)) );
        }else {
            s.push_str( "IX")
        }
         
        Roman{
            s: s
        }
    }
}

impl ToString for Roman {
    fn to_string(&self) -> String {
        self.s.clone()
    }
}