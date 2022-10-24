
pub mod test {
    pub fn study_fn() {
        let mut x: u32 = 10;
        let y: f64 = (22 as f64)/(7 as f64);
        let z = y as u8;

        let mut a = 0b1111_0101u8;

        println!("[sec2_prim_data_types::study_fn] x is {:05}, \ny is {:010.6}, \nz is {}", x, y, z);
        x = 20;
        println!("[sec2_prim_data_types::study_fn] x is {x:05}");

        println!("[sec2_prim_data_types::study_fn] a is {a:08b}");
        a = !a;
        println!("[sec2_prim_data_types::study_fn] NOT : a is {a:08b}");
        let c = a & 0b0000_0010; 
        a = a & 0b1111_1101u8;
        println!("[sec2_prim_data_types::study_fn] AND : a is {a:08b}, c is {c}");
        a = a | 0b0001_0000u8;
        println!("[sec2_prim_data_types::study_fn] OR: a is {a:08b}");
        a = a ^ 0b0101_0101u8;
        println!("[sec2_prim_data_types::study_fn] XOR: a is {a:08b}");
        a = a << 4;
        println!("[sec2_prim_data_types::study_fn] LEFT_SHIFT: a is {a:08b}");
        a = a >> 4;
        println!("[sec2_prim_data_types::study_fn] RIGHT_SHIFT: a is {a:08b}");


        let letter = 'a';
        let number = '1';
        let finger = '\u{261D}';
        println!("[sec2_prim_data_types::study_fn] CHAR: {letter}, {number}, {finger}");
    }
}

/**************
 * Exercise
 ***************/
pub mod exercise {
    pub fn average(a: f32, b:f32, c:f32) -> f32 {
        (a + b + c) / 3.0
    }
}

/**************
 * Unit tests
 ***************/
#[cfg(test)]
mod tests {
    use super::exercise;
    
    #[test]
    fn unittest_average() {
        let fn_return = exercise::average(2.51, 7.5, 15.23);
        let average: f32 = (2.51 + 7.5 + 15.23)/3.0;
        
        assert_eq!(fn_return, average);
    }
}