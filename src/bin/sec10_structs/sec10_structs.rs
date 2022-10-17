
pub mod test {

    /*
     * Shuttle structure
     */
    #[derive(Debug)]
    #[derive(Clone)]
    struct Shuttle {
        name: String,
        crew_size: u8,
        propellent: f64
    }

    impl Shuttle {
        fn new(name: &str, crew: u8, propellent: f64) -> Shuttle {
            Shuttle  {
                name: String::from(name),
                crew_size: crew,
                propellent: propellent
            }
        }

        fn get_name(&self) -> &str {
            &self.name
        }

        fn add_fuel(&mut self, fuel: f64) {
            
            self.propellent = if self.propellent + fuel >= 100.0 {100.0} else {self.propellent + fuel}; 
        }
    }

    /*
     * Color tuple structure
     */
    #[derive(Debug)]
    #[derive(Clone)]
    struct Color(u8, u8, u8);

    #[derive(Debug)]
    #[derive(Clone)]
    struct Point(u32, u32, u32);
    fn get_x(p: Point) -> u32 {
        p.0
    }

    /*
     * Rectangle structure
     */
    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Rectangle {
        width: f64,
        height: f64
    }

    impl Rectangle {
        pub fn new(width_val: f64, height_val: f64) -> Rectangle {
            Rectangle {
                width: width_val,
                height: height_val
            }
        }    
        
        pub fn get_area(&self) -> f64 {
            self.width * self.height
        }

        pub fn scale(&mut self, scale_val: f64) {
            self.width *= scale_val;
            self.height *= scale_val;
        }
    }

    pub fn study_fn() {
        struct_operations();
    }

    fn struct_operations() {
        // Define struct
        let mut vehicle = Shuttle {
            name: String::from("Endeavour"),
            crew_size: 5,
            propellent: 25.5
        };
        println!("[sec10_structs::test::study_fn] Name - {}", vehicle.name);

        vehicle.name = String::from("Atlantis");
        println!("[sec10_structs::test::study_fn] Vehicle - {:?}", vehicle);

        //Update struct
        let vehicle_2 = Shuttle {
            name: String::from("Discovery"), 
            ..vehicle
        };
        vehicle.crew_size = 10;
        // If we are usingthe vehicle instance later, we cannot do this since the pointer is getting moved
        //let mut vehicle_3 = Shuttle{
        //    ..vehicle
        //};
        let vehicle_3 = Shuttle {
            ..vehicle.clone()
        };
        vehicle.crew_size = 10;
        println!("[sec10_structs::test::study_fn] Vehicle - {:?}", vehicle);
        println!("[sec10_structs::test::study_fn] Vehicle 2 - {:?}", vehicle_2);
        println!("[sec10_structs::test::study_fn] Vehicle_3 - {:?}", vehicle_3);
    
        //struct methods
        println!("[sec10_structs::test::study_fn] Vehicle Name - {}", vehicle.get_name());
        println!("[sec10_structs::test::study_fn] Fuel - {}", vehicle.propellent);
        vehicle.add_fuel(20.0);
        println!("[sec10_structs::test::study_fn] Fuel - {}", vehicle.propellent);

        //associate functions
        let vehicle_4 = Shuttle::new("Apollo", 3, 75.0);
        println!("[sec10_structs::test::study_fn] Vehicle_4 - {:?}", vehicle_4);

        //Tuple structs
        let red = Color(255, 0, 0);
        println!("[sec10_structs::test::study_fn] Color - {:?}", red);
        let point = Point(10, 25, 35);
        println!("[sec10_structs::test::study_fn] X Point - {}", get_x(point));
    }
}

/*
 * Exercise
 */
 pub mod exercise {
    use super::test::Rectangle;

    pub fn scaled_area(rect: &Rectangle, scale: f64) -> f64 {
        let mut rec_local = (*rect).clone();
        rec_local.scale(scale);
        rec_local.get_area()
    }
}

/**************
 * Unit tests
 ***************/
#[cfg(test)]
mod tests {
    use super::exercise;
    use super::test;
      
    #[test]
    fn unittest_scaled_area_normal() {
        let rectangle = test::Rectangle::new(2.0, 8.0);
        let scale = 3.0;

        let ret_area = exercise::scaled_area(&rectangle, scale);
        assert_eq!(ret_area, 144.0);
    }

    #[test]
    fn unittest_scaled_area_struct_no_change() {
        let rectangle = test::Rectangle::new(2.0, 8.0);

        let scale = 3.0;
        let ret_area = exercise::scaled_area(&rectangle, scale);
        assert_eq!(ret_area, 144.0);

        let scale = 1.0;
        let ret_area = exercise::scaled_area(&rectangle, scale);
        assert_eq!(ret_area, 16.0);
    }

    #[test]
    fn unittest_scaled_area_zero_width() {
        let rectangle = test::Rectangle::new(0.0, 8.0);
        let scale = 3.0;

        let ret_area = exercise::scaled_area(&rectangle, scale);
        assert_eq!(ret_area, 0.0);
    }

    #[test]
    fn unittest_scaled_area_zero_height() {
        let rectangle = test::Rectangle::new(2.0, 0.0);
        let scale = 3.0;

        let ret_area = exercise::scaled_area(&rectangle, scale);
        assert_eq!(ret_area, 0.0);
    }

    #[test]
    fn unittest_scaled_area_zero_scale() {
        let rectangle = test::Rectangle::new(2.0, 8.0);
        let scale = 0.0;

        let ret_area = exercise::scaled_area(&rectangle, scale);
        assert_eq!(ret_area, 0.0);
    }
}