
pub mod test {
    pub fn study_fn(number: i32) {
        println!("[sec4_functions::test::study_fn]: Number - {}", number);
        println!("[sec4_functions::test::study_fn]: get_sum - {}", get_sum(45, -32));
        println!("[sec4_functions::test::study_fn]: get_sum_and_product - {:?}", get_sum_and_product(45, -32));            
    }

    fn get_sum(number_1: i32, number_2: i32) -> i32 {
        number_1 + number_2
    } 
    fn get_sum_and_product(number_1: i32, number_2: i32) -> (i32, i32) {
        (number_1 + number_2, number_1 * number_2)
    } 
}
/**************
 * Exercise
 */
pub mod exercise { 
    pub fn celcius_to_fahrenheit(celsius: f64) -> f64 {
        (1.8 * celsius) + 32.0
    }
}