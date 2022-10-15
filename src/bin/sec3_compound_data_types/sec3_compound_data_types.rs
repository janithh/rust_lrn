
pub mod test {
    pub fn study_fn() {
        arrays::char_array();

        arrays::number_array();

        arrays::two_dimensional_array();

        tuple();
    }

    // Tuples
    fn tuple() {
        let tuple = (10, 3.14, 'x');
        println!("[sec3_compound_data_types::tuple] Tuple item {}", tuple.2);

        let mut tuple_2: (u8, f32, char) = (15, 5.75, 'a');
        println!("[sec3_compound_data_types::tuple] Tuple 2 item {}", tuple_2.2);
        tuple_2.2 = 'b';
        println!("[sec3_compound_data_types::tuple] Tuple 2 item {}", tuple_2.2);
        let (a, b, c) = tuple_2;
        println!("[sec3_compound_data_types::tuple] Tuple 2 items {}, {}, {}", a, b, c);
    }

    mod arrays {
        //Define character arrays
        pub fn char_array() {
            let mut letters = ['a', 'b', 'c'];
            let first_letter = letters[0];
            println!("[sec3_compound_data_types::arrays::char_array] First letter is {first_letter}");
            letters[0] = 'x';
            println!("[sec3_compound_data_types::arrays::char_array] First letter is {}", letters[0]);
        }

        //Define number array
        pub fn number_array() {
            let mut numbers: [i32; 5];
            numbers = [0; 5];
            println!("[sec3_compound_data_types::arrays::number_array] Last number is {}", numbers[4]);
            numbers = [1, 2, 3, 4, 5];
            let max_index: usize = numbers.len() - 1;
            println!("[sec3_compound_data_types::arrays::number_array] Last number is {}", numbers[max_index]);
        }

        // Two dimensional array
        pub fn two_dimensional_array() {
            let parking_lot = [[1, 2, 3],
                            [4, 5, 6]];
            let number = parking_lot[0][2];
            println!("[sec3_compound_data_types::arrays::two_dimensional_array] Parking lot number is {}", number);

            let _parking_lot_2 = [[0; 5]; 5];
        }
    }   
}