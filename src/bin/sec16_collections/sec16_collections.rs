
pub mod test {
    use std::collections::HashMap;

    pub fn study_fn() {
        test_vector();

        test_hashmap();
    }

    //Vectors
    pub fn test_vector() {
        let mut astronauts: Vec<String> = Vec::new();

        astronauts.push(String::from("Shephard"));
        astronauts.push(String::from("Grissom"));
        astronauts.push(String::from("Glenn"));
        println!("[sec16_collections::test::test_vector] {:?}", astronauts);

        let last_astronaut = astronauts.pop();
        println!("[sec16_collections::test::test_vector] Last - {}", last_astronaut.unwrap());

        //let third_astronaut = &astronauts[2]; /* This will get index out of bound */
        let third_astronaut = match astronauts.get(2) {
            Some(a) => a,
            None => ""
        };
        println!("[sec16_collections::test::test_vector] Third - {}", third_astronaut);
    }
    //HashMaps
    pub fn test_hashmap() {
        let mut missions = HashMap::new();
        missions.insert("Hadfield", 3);
        missions.insert("Hurley", 3);
        missions.insert("Barron", 0);
        println!("[sec16_collections::test::test_hashmap] Missions - {:?}", missions);

        let number = match missions.get("Barron") {
            Some(a) => a,
            None => &-1
        };
        println!("[sec16_collections::test::test_hashmap] Number of Missions - {}", number);

        missions.insert("Barron", 1);
        println!("[sec16_collections::test::test_hashmap] Missions - {:?}", missions);

        missions.entry("Stone").or_insert(2);
        println!("[sec16_collections::test::test_hashmap] Missions - {:?}", missions);
        let stone_val = missions.entry("Stone").or_insert(0);
        *stone_val += 1;
        println!("[sec16_collections::test::test_hashmap] Missions - {:?}", missions);
    }
}

/*
 * Exercise
 */
 pub mod exercise {
    use std::collections::HashMap;
    use std::fs;

    pub fn count_words(file_path: &str) {
        println!("[sec16_collections::exercise::count_word] File Path - {file_path}");
 
        // Read and replace symbols
        let mut contents = fs::read_to_string(file_path).unwrap().to_lowercase();
        contents = contents.replace("\n", " ");
        contents = contents.replace(". ", " ");
        contents = contents.replace(", ", " ");
        contents = contents.replace(" (", " ");
        contents = contents.replace(") ", " ");
        contents = contents.replace(" \"", " ");
        contents = contents.replace("\" ", " ");
        let words = contents.split_whitespace();

        // Read the word and add count to map
        let mut word_map = HashMap::new();
        for word in words {
            let word_count = word_map.entry(word).or_insert(0);
            *word_count += 1;
        }

        // Calculate the word with maximum count
        let mut count = 0;
        let mut max_words: Vec<String> = Vec::new();      
        for (key, value) in word_map.iter() {
            if *value > count {
                count = *value;
                max_words.clear();
                max_words.push(String::from(*key));
            }
            else if *value == count {
                max_words.push(String::from(*key));
            }
        }
        
        println!("[sec16_collections::count_word] Number of Words - {}", word_map.len());
        println!("[sec16_collections::count_word] Max count - {}, Max words - {:?}", count, max_words);
        println!("[sec16_collections::count_word] Words map - {:?}", word_map);
    }
 }

