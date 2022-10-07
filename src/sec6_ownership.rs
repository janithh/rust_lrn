
pub fn study_fn() {
    //shadowing variable
    let planet = "Earth";
    {
        println!("[sec6_ownership::study_fn] Planet - {planet}");
        let mut planet: u8 = 4;
        println!("[sec6_ownership::study_fn] Planet - {planet}");
        planet *= 4;
        println!("[sec6_ownership::study_fn] Planet - {planet}");
    }
    println!("[sec6_ownership::study_fn] Planet - {planet}");

    //String
    let mut message = String::from("Janith");
    message.push_str(" is my name");
    println!("[sec6_ownership::study_fn] String - {message}");

    //********* Conversion *******
    let a: u16 = 500;
    let b: u32 = u32::from(a);
 //   let c: u8 = u8::from(a);

    let d: u32 = a as u32;
    let e: u8 = d as u8;
//    println!("[sec6_ownership::study_fn] {a}, {b}, {c}, {d}, {e}}"); 
    println!("[sec6_ownership::study_fn] {a}, {b}, {d}, {e}"); 

    //Ownership - Moving, cloning and copying data
    //let outer_planet;
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        //let mut inner_planet = "Mercury";
        println!("[sec6_ownership::study_fn] Inner planet : {inner_planet}"); 
        //outer_planet = inner_planet;
        outer_planet = inner_planet.clone();
        println!("[sec6_ownership::study_fn] Inner planet : {inner_planet}"); 
        inner_planet.clear();
        println!("[sec6_ownership::study_fn] Inner planet : {inner_planet}"); 
    }
    println!("[sec6_ownership::study_fn] Outer planet : {outer_planet}");

    let first_val = i32::from(100);
    println!("[sec6_ownership::study_fn] First Value : {first_val}");
    let second_val = first_val;
    println!("[sec6_ownership::study_fn] First Value : {first_val}");
    println!("[sec6_ownership::study_fn] Second Value : {second_val}");

    //transfering ownership
    let rocket_fuel = 10;
    process_fuel(rocket_fuel);
    println!("[sec6_ownership::study_fn] Rocket Fuel : {rocket_fuel}");

    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel_str(rocket_fuel);
    println!("[sec6_ownership::study_fn] Rocket Fuel : {rocket_fuel}");
}

fn process_fuel(mut fuel: i32) {
    fuel += 1;
    println!("[sec6_ownership::process_fuel] Fuel : {fuel}");
}


fn process_fuel_str(fuel: String) -> String {
    println!("[sec6_ownership::process_fuel_str] Fuel : {fuel}");
    String::from("LNG")
}
