use std::fs;

pub fn study_fn(file: String) {
    //Reading files
    println!("[sec9_input_and_output::study_fn] File - {file}");
    let contents = fs::read_to_string(file).unwrap();
    for line in contents.lines() {
        println!("[sec9_input_and_output::study_fn] Planet - {line}");
    }

    //Writing to files
    let mut paragraph = String::new();
    paragraph.push_str("This is Janith\n");
    paragraph.push_str("Writing a sample Rust\n");
    paragraph.push_str("\t Trying to dump to a file\n");
    paragraph.push_str("Thank you !!!");

    let _result = fs::write("./src/paragraph.txt", paragraph);
}

pub fn check_name(file_path: &str, name: &str) -> bool {
    
    println!("[sec9_input_and_output::check_name] File: {}, Name: {}", file_path, name);

    let names = fs::read_to_string(file_path).unwrap();

    for (index, entry_name) in names.lines() {
        if name == entry_name.trim() {
            println!("[sec9_input_and_output::check_name] Name found in the list : {}", name);
            return true;
        }
    }

    println!("[sec9_input_and_output::check_name] Name not found in the list : {}", name);
    return false;
}