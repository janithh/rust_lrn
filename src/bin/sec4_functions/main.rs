
mod sec4_functions;

fn main() {
    /*
     * Section 4 
     */
    sec4_functions::test::study_fn(45);
    let fahrenheit_temp = sec4_functions::exercise::celcius_to_fahrenheit(23.0);
    assert_eq!(fahrenheit_temp, 73.4)
}