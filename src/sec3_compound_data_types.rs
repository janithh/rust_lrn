
pub fn study_fn() {
    let mut letters = ['a', 'b', 'c'];
    let first_letter = letters[0];
    println!("First letter is {first_letter}");
    letters[0] = 'x';
    println!("First letter is {}", letters[0]);

    let mut numbers: [i32; 5];
    numbers = [0; 5];
    println!("Last number is {}", numbers[4]);
    numbers = [1, 2, 3, 4, 5];
    let max_index: usize = numbers.len() - 1;
    println!("Last number is {}", numbers[max_index]);

    let parking_lot = [[1, 2, 3],
                       [4, 5, 6]];
    let number = parking_lot[0][2];
    println!("Parking lot number is {}", number);

    let parking_lot_2 = [[0; 5]; 5];

    // Tuples
    let tuple = (10, 3.14, 'x');
    println!("Tuple item {}", tuple.2);

    let mut tuple_2: (u8, f32, char) = (15, 5.75, 'a');
    println!("Tuple 2 item {}", tuple_2.2);
    tuple_2.2 = 'b';
    println!("Tuple 2 item {}", tuple_2.2);
    let (a, b, c) = tuple_2;
    println!("Tuple 2 items {}, {}, {}", a, b, c);
  
}