
pub mod test {
    use std::fmt;
    use std::any;

    /*
     * Satellite struct
     */
    //#[derive(PartialEq, PartialOrd)]
    #[derive(PartialOrd)]
    pub struct Satellite {
        name: String,
        velocity: f64
    }

    impl Satellite {
        pub fn new(a: String, b: f64) -> Satellite {
            Satellite {
                name: a,
                velocity: b
            }
        }
    }

    impl fmt::Display for Satellite {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Name - {}, Velocity - {}mph", &self.name, &self.velocity)
        }
    }

    impl PartialEq for Satellite {
        fn eq(&self, other: &Self) -> bool {
            self.velocity == other.velocity
        }
    }    

    /*
     * SpaceStation struct
     */
    struct SpaceStation {
        name: String,
        crew_size: u8,
        altitude: u32
    }

    /*
     * Description trait
     */
    trait Description {
        fn describe(&self) -> String {
            format!("This is a flying object") 
        }
    }

    impl Description for Satellite {

    }

    impl Description for SpaceStation {
        fn describe(&self) -> String {
            format!("{} is flying {} miles high with {} crew members", &self.name, &self.altitude, &self.crew_size)
        }
    }

    fn print_type<T: fmt::Debug>(item: T) {
        println!("[sec12_traits::test::print_type] Type of {:?} is {}", item, any::type_name::<T>());
    }

    fn compare_and_print<T, U>(a: T, b: U) 
        where   T: fmt::Display + PartialEq + From<U>,
                U: fmt::Display + PartialEq + Copy  
    {
        if a == T::from(b) {
            println!("[sec12_traits::test::compare_and_print] {} is equal to {}", a, b);
        }
        else {
            println!("[sec12_traits::test::compare_and_print] {} is NOT equal to {}", a, b);   
        }
    }

    fn get_displayable() -> impl fmt::Display{
        13
    }

    pub fn study_fn() {

        // Implement traits, Default traits
        let hubble = Satellite {
            name: String::from("Hubble"),
            velocity: 4.72
        };

        let iss = SpaceStation {
            name: String::from("International Space Station"),
            crew_size: 6,
            altitude: 254
        };

        //println!("[sec12::study_fn] ");
        println!("[sec12_traits::test::study_fn] Hubble - {}", hubble.describe());
        println!("[sec12_traits::test::study_fn] ISS - {}", iss.describe());

        //Derive traits
        let gps = Satellite {
            name: String::from("GPS Satellite"),
            velocity: 7.5
        };
        println!("[sec12_traits::test::study_fn] Hubble == GPS is {}", hubble == gps);
        println!("[sec12_traits::test::study_fn] Hubble > GPS is {}", hubble > gps);

        //Trait bounds
        print_type(13);
        print_type(13.0);
        print_type("13");
        print_type(String::from("13"));
        print_type([1, 13, 7]);
        print_type((1, 12.5, "test"));

        //Multiple trait bounds
        compare_and_print(1.0, 1);

        //Return types with multiple 
        println!("[sec12_traits::test::study_fn] Get Display - {}", get_displayable());
    }
}
/*
 * Exercise
 */
pub mod exercise {
    use std::any;
    use super::test::Satellite;

    pub fn display_object() -> String {
        let hubble = Satellite::new(String::from("Hubble"), 4.72); 
        format!("Type - {}, {}", any::type_name::<Satellite>(), hubble)
    }

    pub fn compare_objects() -> bool {
        let hubble = Satellite::new(String::from("Hubble"), 4.72); 
        let gps = Satellite::new(String::from("GPS"), 7.83);
        
        hubble == gps
    }
}