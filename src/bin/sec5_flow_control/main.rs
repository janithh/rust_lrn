
mod sec5_flow_control;

fn main() {
    /*
     * Section 5 
     */
    sec5_flow_control::test::study_fn();

    let numbers: [i32; 14] = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let (min, max, mean) = sec5_flow_control::exercise::calculate_stats(&numbers);

    println!("[sec5_flow_control::main] Min - {min}, Max - {max}, Mean - {mean}");
}