
 use std::collections::HashMap;
 use std::fs;

pub fn study_fn() {
    //Vectors
    let mut astronauts: Vec<String> = Vec::new();

    astronauts.push(String::from("Shephard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));
    println!("[sec6_collections::study_fn] {:?}", astronauts);

    let last_astronaut = astronauts.pop();
    println!("[sec6_collections::study_fn] Last - {}", last_astronaut.unwrap());

    //let third_astronaut = &astronauts[2]; /* This will get index out of bound */
    let third_astronaut = match astronauts.get(2) {
        Some(a) => a,
        None => ""
    };
    println!("[sec6_collections::study_fn] Third - {}", third_astronaut);

    //HashMaps
    let mut missions = HashMap::new();
    missions.insert("Hadfield", 3);
    missions.insert("Hurley", 3);
    missions.insert("Barron", 0);
    println!("[sec6_collections::study_fn] Missions - {:?}", missions);

    let number = match missions.get("Barron") {
        Some(a) => a,
        None => &-1
    };
    println!("[sec6_collections::study_fn] Number of Missions - {}", number);

    missions.insert("Barron", 1);
    println!("[sec6_collections::study_fn] Missions - {:?}", missions);

    missions.entry("Stone").or_insert(2);
    println!("[sec6_collections::study_fn] Missions - {:?}", missions);
    let stone_val = missions.entry("Stone").or_insert(0);
    *stone_val += 1;
    println!("[sec6_collections::study_fn] Missions - {:?}", missions);   
}

/*
 * Exercise
 */
 pub fn count_words(file_path: &str) {
    println!("[sec16_collections::count_word] File Path - {file_path}");
    let mut contents = fs::read_to_string(file_path).unwrap().to_lowercase();
    contents = contents.replace("\n", " ");
    contents = contents.replace(". ", " ");
    contents = contents.replace(", ", " ");
    contents = contents.replace(" (", " ");
    contents = contents.replace(") ", " ");
    contents = contents.replace(" \"", " ");
    contents = contents.replace("\" ", " ");
    let words = contents.split_whitespace();
    let mut word_map = HashMap::new();

    for word in words {
        let word_count = word_map.entry(word).or_insert(0);
        *word_count += 1;
    }

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


