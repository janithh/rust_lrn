
pub fn study_fn() {
    //Borrowing references
    let mut fuel = String::from("RP-1");
    let length = process_fuel(&mut fuel);
    println!("[sec7_references::study_fn] Fuel - {}, Length - {}", fuel, length);

    //Dangling 
    let fuel = produce_fuel();
    println!("[sec7_references::study_fn] Fuel - {}", fuel);

    //Slices
    let message = String::from("Greetings from earth");
    println!("[sec7_references::study_fn] Message - {}", message);
    let word = &message[..15];
    //let word = &message[15..20];
    //let word = &message[15..];
    println!("[sec7_references::study_fn] Word - {}", word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..2];
    println!("[sec7_references::study_fn] Planets - {:?}", inner_planets);

    //Slice as function params
    let first_word = get_first_word(&message[10..]);
    println!("[sec7_references::study_fn] First word - {}", first_word);
}

fn process_fuel(propellent: &mut String) -> usize {
    println!("[sec7_references::process_fuel] Fuel - {}", propellent);
    propellent.push_str(" Super");
    let length = propellent.len();
    length
}

/*
 * This will not work since scope of new_fuel is within the fuction. Instead return the string itself
fn produce_fuel() -> &String { 
    let new_fuel = String::from("LNG");
    &new_fuel
}*/
fn produce_fuel() -> String { 
    let new_fuel = String::from("LNG");
    new_fuel
}

fn get_first_word(message: &str) -> &str {
    let message_bytes = message.as_bytes();

    for (index, &item) in message_bytes.iter().enumerate() {
        if item == b' ' {
            return &message[..index];
        }
    }

    &message

} 

/*
 * Exercise
 */
pub fn trim_spaces(data: &str) -> &str {
    let data_byte = data.as_bytes();
    //let data_byte = data.chars();
    let first_index = {
        let mut index = 0; 
        loop {
            if index == data_byte.len() || data_byte[index] != b' '  {
                break index;
            }
            index += 1;
        }
    };

    if first_index == data.len() {
        return "";
    }

    let last_index = {
        let mut index = data_byte.len() - 1;
        loop {
            if data_byte[index] != b' ' {
                break index;
            }
            index -= 1;
        }
    };

    &data[first_index..last_index + 1]
}