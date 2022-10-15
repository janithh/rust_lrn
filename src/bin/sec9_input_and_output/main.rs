use std::env;

mod sec9_input_and_output;

fn main() {
    /*
     * Section 9
     */  
    let req_arguments = 2;
    let num_arguments = env::args().len();
    if num_arguments != req_arguments {
        println!("[Main::sec9] Invaid number of arguments. Required - {req_arguments}, Received - {num_arguments}");
        return;  
    }
    for (index, argument) in env::args().enumerate() {
        println!("[Main::sec9] Index - {index}, Argument - {argument}");
    }
    let file_name = env::args().nth(1).unwrap();
    sec9_input_and_output::test::study_fn(file_name);
    
    sec9_input_and_output::exercise::check_name("./files/moonwalkers.txt", "Duke");  
}