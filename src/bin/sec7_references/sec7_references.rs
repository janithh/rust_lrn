
pub mod test {
    pub fn study_fn() {
        borrow_reference();

        dangling();

        slices::string_slices();

        slices::array_slices();

        slices::slices_as_param();
    }
    
    fn borrow_reference() {
        //Borrowing references
        let mut fuel = String::from("RP-1");
        let length = process_fuel(&mut fuel);
        println!("[sec7_references::test::borrow_reference] Fuel - {}, Length - {}", fuel, length);
    }

    fn dangling() {
        //Dangling 
        let fuel = produce_fuel();
        println!("[sec7_references::test::dangling] Fuel - {}", fuel);
    }

    mod slices {
        pub fn string_slices()
        {
            //Slices
            let message = String::from("Greetings from earth");
            println!("[sec7_references::test::slices::string_slices] Message - {}", message);
            let word = &message[..15];
            //let word = &message[15..20];
            //let word = &message[15..];
            println!("[sec7_references::test::slices::string_slices] Word - {}", word);
        }

        pub fn array_slices() {
            let planets = [1, 2, 3, 4, 5, 6, 7, 8];
            let inner_planets: &[i32] = &planets[..2];
            println!("[sec7_references::test::slices::array_slices] Planets - {:?}", inner_planets);
        }

        pub fn slices_as_param() {
            //Slice as function params
            let message = String::from("Greetings from earth");
            let first_word = super::get_first_word(&message[10..]);
            println!("[sec7_references::test::slices::slices_as_param] First word - {}", first_word);
        }
    }

    fn process_fuel(propellent: &mut String) -> usize {
        println!("[sec7_references::test::process_fuel] Fuel - {}", propellent);
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
}

/*
 * Exercise
 */
pub mod exercise {
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
}