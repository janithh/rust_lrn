
pub mod test {
    use std::env;
    use std::fs;
    use std::io;

    pub fn study_fn() {
        env::set_var("RUST_BACKTRACE", "0");

        unrecoverable_error();

        test_result_t_e_enum();
    }

    fn unrecoverable_error() {
        //Unrecoverable error
        //panic!("[sec5_error_handling::study_fn] Test Panic");
        //println!("[sec5_error_handling::study_fn] ");

        let count_down = [5, 4, 3, 2, 1, 0];
        for count in count_down.iter() {
            println!("[sec15_error_handling::test::study_fn] Count - {count}");
            //let x = 1 / count; /* Will end up in Divide by zero panic */
        }
    }

    fn test_result_t_e_enum() {
        //Result<T, E> enum
        let content = fs::read_to_string("paragraph.txt");
        //let content_2 = fs::read_to_string("./files/paragraph.txt");
        println!("[sec15_error_handling::test::test_result_t_e_enum] Err - {:?}", content);
        //println!("[sec15_error_handling::test::study_fn] Ok - {:?}", content_2);
        //let data = content_2.unwrap();
        //println!("[sec15_error_handling::test::study_fn] Content - {data}");
        
        //Matching results
        let result = match content {
            Ok(a)  =>  a,
            Err(a) =>  match a.kind() {
                io::ErrorKind::NotFound => String::from("File not found"),
                io::ErrorKind::PermissionDenied => String::from("Permission denied"),
                _ => panic!("[sec15_error_handling::test::test_result_t_e_enum] Unhandled file open error")
            },
        };
        println!("[sec15_error_handling::test::test_result_t_e_enum] Result - {result}");
    }
}
    /*
    * Exercise
    */
pub mod exercise {
    use std::env;
    use std::io;
    use rand::prelude::*;
    
    pub fn guess_number() {
        env::set_var("RUST_BACKTRACE", "1");

        let number = thread_rng().gen_range(0..100);
        let mut guess_count = 1;
            
        loop {
            let mut buffer = String::new();
            println!("[sec15_error_handling::exercise::guess_number] Enter a number (0 - 100) or X to exit (Guess - {guess_count}) :");
            let _status = match io::stdin().read_line(&mut buffer) {
                Ok(a) => a,
                Err(_) => {
                        println!("[sec15_error_handling::exercise::guess_number] Input error");
                        continue
                    }         
            };

            if buffer.trim().to_uppercase() == "X" {
                break;
            }
            else if buffer.trim().is_empty() == true {
                continue;
            }
            else {
                let input_number: u32 = match buffer.trim().parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("[sec15_error_handling::exercise::guess_number] Invalid input !!!");
                        continue
                    }
                
                };

                if input_number == number {
                    println!("[sec15_error_handling::exercise::guess_number] Good guess !!!");
                    break;
                }
                else if input_number < number {
                    println!("[sec15_error_handling::exercise::guess_number] Your input is less !!!");
                }
                else {
                    println!("[sec15_error_handling::exercise::guess_number] Your input is higher !!!");
                }
            }

            guess_count += 1;
        }  
    }
}