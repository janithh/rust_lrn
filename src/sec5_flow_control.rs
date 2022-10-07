pub fn study_fn() {
    let x = 3;

    // Conditions
    if x + 1 == 5 {
        println!("[sec5_flow_control::study_fn] x is 5");
    } else if x + 1 == 4 {
        println!("[sec5_flow_control::study_fn] x is 3");
    } else {
        println!("[sec5_flow_control::study_fn] x is NOT 5. It is {x}");
    }

    //Conditional assignment
    let make_odd = true;
    let y = if make_odd {1} else {2};
    println!("[sec5_flow_control::study_fn] y is {y}");
    
    //Loop
    let mut count_loop = 0;
    let loop_result = loop {
        println!("[sec5_flow_control::study_fn] count_loop : {count_loop}");
        count_loop += 1;

        if count_loop > 10 {
            break count_loop;
        }
    };
    println!("[sec5_flow_control::study_fn] loop_result : {loop_result}");

    //While loop
    let letters = ['J', 'A', 'N', 'I', 'T', 'H'];
    let mut count_while = 0;

    while count_while < letters.len() {
        println!("[sec5_flow_control::study_fn] letter : {}", letters[count_while]); 
        count_while += 1;     
    }

    //For loop
    for (index, letter) in letters.iter().enumerate() {
        println!("[sec5_flow_control::study_fn] in For- Index : {}, letter : {}", index, letter); 
    }

    for number in 0..5 {
        println!("[sec5_flow_control::study_fn] in For- Number : {}", number); 

    }

    //Nested loop
    let mut matrix = [[1, 2, 3],
                  [4, 5, 6],
                  [7, 8, 9]];

    println!("[sec5_flow_control::study_fn] Matrix : ");               
    for row in matrix {  
        for mut value in row {
            value *= 2;
            print!("\t {}", value);           
        }
        println!("");
    }
    println!("[sec5_flow_control::study_fn] Matrix : ");               
    for row in matrix {
        for value in row {
            print!("\t {}", value);           
        }
        println!("");
    }

    println!("[sec5_flow_control::study_fn] Matrix : ");  
    for row in matrix.iter_mut() {
        for value in row.iter_mut() {
            *value *= 2;
            print!("\t {}", value);           
        }
        println!("");
    }
    println!("[sec5_flow_control::study_fn] Matrix : "); 
    for row in matrix {
        for value in row {
            print!("\t {}", value); 
        }
        println!("");
    }

}


/**************
 * Exercise
 */
pub fn calculate_stats() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    let array_size = numbers.len();

    if array_size == 0 {
        return;
    }

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    for number_item in numbers {
        if max < number_item {
            max = number_item;
        }
        if min > number_item {
            min = number_item;
        }

        mean += number_item as f64;
    }

    mean /= array_size as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);

    println!("Test passed !!!");
}