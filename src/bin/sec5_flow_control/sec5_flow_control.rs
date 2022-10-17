
pub mod test {
    pub fn study_fn() {
        conditions();

        loops::loop_fn();

        loops::while_loop();

        loops::for_loop();

        loops::nested_loop();
    }

    fn conditions() {
        let x = 3;

        // Conditions
        if x + 1 == 5 {
            println!("[sec5_flow_control::test::conditions] x is 5");
        } else if x + 1 == 4 {
            println!("[sec5_flow_control::test::conditions] x is 3");
        } else {
            println!("[sec5_flow_control::test::conditions] x is NOT 5. It is {x}");
        }

        //Conditional assignment
        let make_odd = true;
        let y = if make_odd {1} else {2};
        println!("[sec5_flow_control::test::conditions] y is {y}");
    }

    mod loops {
        pub fn loop_fn() {
            //Loop
            let mut count_loop = 0;
            let loop_result = loop {
                println!("[sec5_flow_control::loop::loop_fn] count_loop : {count_loop}");
                count_loop += 1;

                if count_loop > 10 {
                    break count_loop;
                }
            };
            println!("[sec5_flow_control::loop::loop_fn] loop_result : {loop_result}");
        }
        
        pub fn while_loop() {
            //While loop
            let letters = ['J', 'A', 'N', 'I', 'T', 'H'];
            let mut count_while = 0;

            while count_while < letters.len() {
                println!("[sec5_flow_control::loop::while_loop] letter : {}", letters[count_while]); 
                count_while += 1;     
            }
        }

        pub fn for_loop() {
            //For loop
            let letters = ['J', 'A', 'N', 'I', 'T', 'H'];
            for (index, letter) in letters.iter().enumerate() {
                println!("[sec5_flow_control::loop::for_loop] in For- Index : {}, letter : {}", index, letter); 
            }

            for number in 0..5 {
                println!("[sec5_flow_control::loop::for_loop] in For- Number : {}", number); 

            }
        }

        pub fn nested_loop() {
            //Nested loop
            let mut matrix = [[1, 2, 3],
                        [4, 5, 6],
                        [7, 8, 9]];

            println!("[sec5_flow_control::loop::nested_loop] Matrix : ");               
            for row in matrix {  
                for mut value in row {
                    value *= 2;
                    print!("\t {}", value);           
                }
                println!("");
            }
            println!("[sec5_flow_control::loop::nested_loop] Matrix : ");               
            for row in matrix {
                for value in row {
                    print!("\t {}", value);           
                }
                println!("");
            }

            println!("[sec5_flow_control::loop::nested_loop] Matrix : ");  
            for row in matrix.iter_mut() {
                for value in row.iter_mut() {
                    *value *= 2;
                    print!("\t {}", value);           
                }
                println!("");
            }
            println!("[sec5_flow_control::loop::nested_loop] Matrix : "); 
            for row in matrix {
                for value in row {
                    print!("\t {}", value); 
                }
                println!("");
            }
        }
    }
}


/**************
 * Exercise
 */
pub mod exercise {
    pub fn calculate_stats(numbers: &[i32]) -> (i32, i32, f64) {
        let mut max: i32;
        let mut min: i32;
        let mut mean: f64;

        let array_size = numbers.len();

        if array_size == 0 {
            return (0, 0, 0.0);
        }

        max = numbers[0];
        min = numbers[0];
        mean = 0.0;

        for number_item in numbers {
            if max < *number_item {
                max = *number_item;
            }
            if min > *number_item {
                min = *number_item;
            }

            mean += *number_item as f64;
        }

        mean /= array_size as f64;

        (min, max, mean)
    }
}

/**************
 * Unit tests
 ***************/
#[cfg(test)]
mod tests {
    use super::exercise;
     
    #[test]
    fn unittest_calculate_stats_normal() {

        let array: [i32; 7] = [0, 1, 2, 2, 1, 7, 3];
        let (min, max, mean) = exercise::calculate_stats(&array);
         
        assert_eq!(min, 0);
        assert_eq!(max, 7);
        assert_eq!(mean, 16.0/7.0);
    }

    #[test]
    fn unittest_calculate_stats_zeros() {

        let array: [i32; 5] = [0, 0, 0, 0, 0];
        let (min, max, mean) = exercise::calculate_stats(&array);
         
        assert_eq!(min, 0);
        assert_eq!(max, 0);
        assert_eq!(mean, 0.0);
    }

    #[test]
    fn unittest_calculate_stats_single() {

        let array: [i32; 1] = [5];
        let (min, max, mean) = exercise::calculate_stats(&array);
         
        assert_eq!(min, 5);
        assert_eq!(max, 5);
        assert_eq!(mean, 5.0);
    }

    #[test]
    fn unittest_calculate_stats_empty() {

        let array: [i32; 0] = [];
        let (min, max, mean) = exercise::calculate_stats(&array);
         
        assert_eq!(min, 0);
        assert_eq!(max, 0);
        assert_eq!(mean, 0.0);
    }
}

 