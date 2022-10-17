
pub mod test {
    use std::fs;

    pub fn study_fn(file: String) {
        read_file(file);

        write_file();
    }

    fn read_file(file: String) {
        //Reading files
        println!("[sec9_input_and_output::test::study_fn] File - {file}");
        let contents = fs::read_to_string(file).unwrap();
        for line in contents.lines() {
            println!("[sec9_input_and_output::test::study_fn] Planet - {line}");
        }
    }

    fn write_file() {
        //Writing to files
        let mut paragraph = String::new();
        paragraph.push_str("This is Janith\n");
        paragraph.push_str("Writing a sample Rust\n");
        paragraph.push_str("\t Trying to dump to a file\n");
        paragraph.push_str("Thank you !!!");

        let _result = fs::write("./files/paragraph.txt", paragraph);
    }
}

/*
 * Exercise
 */
 pub mod exercise {
    use std::fs;

    pub fn check_name(file_path: &str, name: &str) -> bool {
        
        println!("[sec9_input_and_output::exercise::check_name] File: {}, Name: {}", file_path, name);

        let names = fs::read_to_string(file_path).unwrap();

        for entry_name in names.lines() {
            if name == entry_name.trim() {
                println!("[sec9_input_and_output::exercise::check_name] Name found in the list : {}", name);
                return true;
            }
        }

        println!("[sec9_input_and_output::exercise::check_name] Name not found in the list : {}", name);
        return false;
    }
}

/**************
 * Unit tests
 ***************/
#[cfg(test)]
mod tests {
    use super::exercise;
     
    #[test]
    fn unittest_check_name_normal() {
        let file_path = "./files/moonwalkers.txt";
        let name = "Duke";
        let fn_return = exercise::check_name(file_path, name);
         
        assert_eq!(fn_return, true);
    }

    #[test]
    #[should_panic]
    fn unittest_check_name_invalid_path() {
        let file_path = "./src/moonwalkers.txt";
        let name = "Duke";
        let fn_return = exercise::check_name(file_path, name);
         
        assert_eq!(fn_return, false);
    }

    #[test]
    #[should_panic]
    fn unittest_check_name_empty_path() {
        let file_path = "";
        let name = "Duke";
        let fn_return = exercise::check_name(file_path, name);
         
        assert_eq!(fn_return, false);
    }

    #[test]
    fn unittest_check_name_not_found_name() {
        let file_path = "./files/moonwalkers.txt";
        let name = "Oliver";
        let fn_return = exercise::check_name(file_path, name);
         
        assert_eq!(fn_return, false);
    }

    #[test]
    fn unittest_check_name_empty_name() {
        let file_path = "./files/moonwalkers.txt";
        let name = "";
        let fn_return = exercise::check_name(file_path, name);
         
        assert_eq!(fn_return, false);
    }
}