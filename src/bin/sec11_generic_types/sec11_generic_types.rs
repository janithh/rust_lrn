pub mod test {
    use std::mem;

    /*
     * Rectangle struct
     */
    #[derive(Debug)]
    struct Rectangle<T, U> {
        width: T,
        height: U
    }

    impl<T, U> Rectangle<T, U> {
        fn get_width(&self) -> &T {
            &self.width
        }
    }

    impl Rectangle<u8, u8> {
        fn get_perimeter(&self) -> u8 {
            2 * (self.width + self.height)
        }
    }

    /*
     * Shuttle struct
     */
    struct Shuttle {
        name: String,
        crew_size: u8,
        propellent: f64
    }

    pub fn study_fn() {
        generic_struct();

        generic_func();

        box_data();
    }

    fn generic_struct() {
        //Generic structs and methods
        let rect = Rectangle {
            width: 12,
            height: 10
        };

        println!("[sec_generic_types::study_fn] Rectangle - {:?}", rect);
        println!("[sec_generic_types::study_fn] Rectangle width - {}", rect.get_width());
        println!("[sec_generic_types::study_fn] Rewctangle perimeter - {}", rect.get_perimeter());
    }

    fn generic_func() {
        //Generic functions
        println!("[sec_generic_types::study_fn] Get the largest - {}", get_largest(2, 1));
        println!("[sec_generic_types::study_fn] Get the largest - {}", get_largest(2.75, 1.24));
        println!("[sec_generic_types::study_fn] Get the largest - {}", get_largest("Two point five", "Three point two six"));
    }

    fn box_data() {
        //Box data type
        let vehicle = Shuttle {
            name: String::from("Atlantis"),
            crew_size: 7,
            propellent: 85000.0
        };
        println!("[sec_generic_types::study_fn] Shuttle mem size on stack - {}", mem::size_of_val(&vehicle));
        
        let boxed_vehicle = Box::new(vehicle);
        println!("[sec_generic_types::study_fn] Boxed shuttle mem size on stack - {}", mem::size_of_val(&boxed_vehicle));
        println!("[sec_generic_types::study_fn] Boxed shuttle mem size on heap - {}", mem::size_of_val(&*boxed_vehicle));

        let unboxed_vehicle = *boxed_vehicle;
        println!("[sec_generic_types::study_fn] Unboxed shuttle mem size on stack - {}", mem::size_of_val(&unboxed_vehicle));
    }

    fn get_largest<T: PartialOrd>(a: T, b: T) -> T {
        if a >= b {a} else {b}
    }
}

/*
 * Exercise
 */
pub mod exercise {
    use std::ops;

    pub fn sum_boxes<T>(first :Box<T>, second: Box<T>) -> Box<T> 
        where T: ops::Add<Output = T>
    {
        Box::new(*first + *second)
    }
}