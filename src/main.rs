#[allow(unused_imports)] /*Disable unused_imports warning*/
#[allow(dead_code)] /*Disable dead_code lint*/

use std::env;

//mod sec2_prim_data_types;
//mod sec3_compound_data_types;
//mod sec4_functions;
//mod sec5_flow_control;
//mod sec6_ownership;
//mod sec7_references;
//mod sec8_modules;
//mod sec9_input_and_output;
//mod sec10_structs;
//mod sec11_generic_types;
//mod sec12_traits;
//mod sec13_lifetimes;
//mod sec14_enums;
//mod sec15_error_handling;
mod sec16_collections;

/*
 * Main
 */
fn main() {
    /*
     * Section 2 
     */
    //sec2_prim_data_types::study_fn();
    //sec2_prim_data_types::average();

    /*
     * Section 3 
     */
    //sec3_compound_data_types::study_fn();

    /*
     * Section 4 
     */
    //sec4_functions::study_fn(45);
    //let fahrenheit_temp = sec4_functions::celcius_to_fahrenheit(23.0);
    //assert_eq!(fahrenheit_temp, 73.4)

    /*
     * Section 5 
     */
    //sec5_flow_control::study_fn();

    /*
     * Section 6 
     */
    //sec6_ownership::study_fn();

    /*
     * Section 7 
     */
     //sec7_references::study_fn();
    //let exercise_message = String::from("         ");
    //println!("[sec7_references::study_fn] Trim message - \"{}\"", sec7_references::trim_spaces(&exercise_message));


    /*
     * Section 8
     */
    //sec8_modules::study_fn();
    //sec8_modules::guess_number();

    /*
     * Section 9
     */
     /*
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
    sec9_input_and_output::study_fn(file_name);
    */
    //sec9_input_and_output::check_name("./src/moonwalkers.txt", "Duke");    

    /*
     * Section 10
     */
     //sec10_structs::study_fn();
    //sec10_structs::test_rectangle();

    /*
     * Section 11
     */
    //sec11_generic_types::study_fn();

    /*
     * Section 12
     */
    //sec12_traits::study_fn();
    //println!("[Main] {}", sec12_traits::display_object());
    //println!("[Main] Compare results - {}", sec12_traits::compare_objects());

    /*
     * Section 13
     */
    //sec13_lifetimes::study_fn();

    /*
     * Section 14
     */
    //sec14_enums::study_fn();
    //sec14_enums::get_location();

    /*
     * Section 15
     */
    //sec15_error_handling::study_fn();
    //sec15_error_handling::guess_number();

    /*
     * Section 16
     */
    //sec16_collections::study_fn();
    let arguments_expected = 2;
    let arguments_num = env::args().len();

    assert_eq!(arguments_expected, arguments_num, "[Main] Invalid arguments : Expected - {arguments_expected}, Received - {arguments_num}");

    let file_path = env::args().nth(1).unwrap();
    sec16_collections::count_words(&file_path);

}

