use std::fmt;

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(a) => 2.0 * std::f64::consts::PI * a,
            Shape::Rectangle(a, b) => 2.0 * (a + b),
            Shape::Triangle (a, b, c) => a + b + c
        }
    }
}

pub fn study_fn() {
    //Define Enums
    let my_shape = Shape::Circle(2.34);
    //println!("[sec14_study_fn]")
    println!("[sec14_enums::study_fn] My Shape - {:?}", my_shape);

    //Match operator
    match my_shape {
        Shape::Circle(a) => println!("[sec14_enums::study_fn] Circle : Radius - {}", a),
        Shape::Rectangle(a, b) => println!("[sec14_enums::study_fn] Rectangle : Side 1 - {}, Side 2 - {}", a, b),
        Shape::Triangle(a, b, c) => println!("[sec14_enums::study_fn] Triangle : Side 1 - {}, Side 2 - {}, Side 3 - {}", a, b, c)
    }

    //Match with default
    let my_number = 10;
    let result = match my_number {
        10 => "Ten",
        20 => "Twenty",
        30 => "Thirty",
        _ => {
                println!("[sec14_enums::study_fn] No match : Number - {}", my_number);
                "Invalid number"
        }
    };
    println!("[sec14_enums::study_fn] My number string - {}", result);

    //Enum methods
    let perimeter = my_shape.get_perimeter();
    println!("[sec14_enums::study_fn] Perimeter - {}", perimeter);  

    //Option<T> enum
    let count_down =[5, 4, 3, 2, 1]; 
    let number = count_down.get(5);
    let number = number.unwrap_or(&-1); 
    println!("[sec14_enums::study_fn] Number - {}", number);  

    //Matching option<T>
    let number_match = count_down.get(2);
    let number_match = match number_match {
        Some(number_match) => number_match + 0,
        None => -1
    };
    println!("[sec14_enums::study_fn] Match Number - {}", number_match);  

    //if-let syntax
    let number = Some(13);
    let test = 5;

    /*match number {
        Some(13) => println!("[sec14_study_fn] Thirteen"),
        _ => println!("[sec14_study_fn] Other")
    };*/
    //if let Some(13) = number {
    if number == Some(13) {
        println!("[sec14_enums::study_fn] Thirteen");
    }
    else {
        println!("[sec14_enums::study_fn] Other");
    }

    if let 5 = test {
        println!("[sec14_enums::study_fn] OK");
    }
}

/*
 * Exercise
 *
 */
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Location::Unknown => write!(f, "[sec14_enums::Location::display] Location Unknown"),
            Location::Anonymous => write!(f, "[sec14_enums::Location::display] Location Anonymous"),
            Location::Known(a, b) => write!(f, "[sec14_enums::Location::display] Location - ({a}, {b})")
        }
    }
}

pub fn get_location() {
    let location = Location::Known(2.754, 5.763);
    println!("{location}");
}