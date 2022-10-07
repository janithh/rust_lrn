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

#[derive(Debug)]
#[derive(Clone)]
struct Color(u8, u8, u8);

#[derive(Debug)]
#[derive(Clone)]
struct Point(u32, u32, u32);
fn get_x(p: Point) -> u32 {
    p.0
}

pub struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn new(width_val: f64, height_val: f64) -> Rectangle {
        Rectangle {
            width: width_val,
            height: height_val
        }
    }    
    
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, scale_val: f64) {
        self.width *= scale_val;
        self.height *= scale_val;
    }
}

pub fn study_fn() {
    // Define struct
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 5,
        propellent: 25.5
    };
    println!("[sec10_structs::study_fn] Name - {}", vehicle.name);

    vehicle.name = String::from("Atlantis");
    println!("[sec10_structs::study_fn] Vehicle - {:?}", vehicle);

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
    println!("[sec10_structs::study_fn] Vehicle - {:?}", vehicle);
    println!("[sec10_structs::study_fn] Vehicle 2 - {:?}", vehicle_2);
    println!("[sec10_structs::study_fn] Vehicle_3 - {:?}", vehicle_3);

    //struct methods
    println!("[sec10_structs::study_fn] Vehicle Name - {}", vehicle.get_name());
    println!("[sec10_structs::study_fn] Fuel - {}", vehicle.propellent);
    vehicle.add_fuel(20.0);
    println!("[sec10_structs::study_fn] Fuel - {}", vehicle.propellent);

    //associate functions
    let vehicle_4 = Shuttle::new("Apollo", 3, 75.0);
    println!("[sec10_structs::study_fn] Vehicle_4 - {:?}", vehicle_4);

    //Tuple structs
    let red = Color(255, 0, 0);
    println!("[sec10_structs::study_fn] Color - {:?}", red);
    let point = Point(10, 25, 35);
    println!("[sec10_structs::study_fn] X Point - {}", get_x(point));

}

/*
 * Exercise
 */
 pub fn test_rectangle() {
    let mut rectangle = Rectangle::new(10.2, 11.2);
    println!("[sec10_structs::test_rectangle] Area : {}", rectangle.get_area());
    rectangle.scale(5.1);
    println!("[sec10_structs::test_rectangle] Area : {}", rectangle.get_area());
 }