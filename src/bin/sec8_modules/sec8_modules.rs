
pub mod test {
    use std::io;
    use rand::prelude::*;

    pub fn study_fn() {
        get_input();

        random_number();
    }

    fn get_input() {
        let mut buffer = String::new();
        println!("[sec8_modules::test::get_input] Enter input :");
        let _result = io::stdin().read_line(&mut buffer);
        println!("[sec8_modules::test::get_input] Input : {buffer}");

        //trim - remove return at the end
        //let number = buffer.trim().parse::<i32>().unwrap();
        let number: i32 = buffer.trim().parse().unwrap();
        println!("[sec8_modules::test::get_input] Number : {number}");
    }

    fn random_number() {
        let rand_number = random::<f64>();
        println!("[sec8_modules::test::random_number] Random number : {rand_number}");

        let rand_number = thread_rng().gen_range(1..10);
        println!("[sec8_modules::test::random_number] Random number : {rand_number}");
    }
}

/*
 * Exercise
 */
pub mod exercise {
    use std::io;
    use std::env;
    use rand::prelude::*;

    pub fn guess_number() {
        env::set_var("RUST_BACKTRACE", "1");
        let number = thread_rng().gen_range(0..100);
        let mut guess_count = 1;
            
        loop {
            let mut buffer = String::new();
            println!("[sec8_modules::exercise::guess_number] Enter a number (0 - 100) or X to exit (Guess - {guess_count}) :");
            io::stdin().read_line(&mut buffer).expect("[sec8_modules::exercise::guess_number] Invalid input !!!");
            

            if buffer.trim().to_uppercase() == "X" {
                break;
            }
            else if buffer.trim().is_empty() == true {
                continue;
            }
            else {
                let input_number: u32 = buffer.trim().parse().expect("[sec8_modules::exercise::guess_number] Invalid input number");

                if input_number == number {
                    println!("[sec8_modules::exercise::guess_number] Good guess !!!");
                    break;
                }
                else if input_number < number {
                    println!("[sec8_modules::exercise::guess_number] Your input is less !!!");
                }
                else {
                    println!("[sec8_modules::exercise::guess_number] Your input is higher !!!");
                }
            }

            guess_count += 1;
        }  
    }
}