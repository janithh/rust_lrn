
mod sec7_references;

fn main() {
    /*
     * Section 7 
     */
    sec7_references::test::study_fn();
    
    let exercise_message = String::from("         ");
    println!("[sec7_references::study_fn] Trim message - \"{}\"", sec7_references::exercise::trim_spaces(&exercise_message));
}