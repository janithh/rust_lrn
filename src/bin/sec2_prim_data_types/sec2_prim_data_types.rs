
pub mod test {
    pub fn study_fn() -> bool {
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

        return true;
    }
}

    /**************
    * Exercise
    ***************/
pub mod exercise {
    pub fn average() {
        let a = 13;
        let b = 2.3;
        let c: f32 = 120.0;


        //let average = (f64::from(a) + f64::from(b) + f64::from(c)) / 3.0; /*** My answer ***/
        let average = (a as f64 + b as f64  + c as f64) / 3.0; 
    
        println!("[sec2_prim_data_types::average] Average: {}", average);

        assert_eq!(average, 45.1);
        println!("[sec2_prim_data_types::average] Test Passed ! ");
    }
}