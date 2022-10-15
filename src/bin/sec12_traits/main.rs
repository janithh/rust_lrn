
mod sec12_traits;

fn main() {
    /*
     * Section 12
     */
    sec12_traits::test::study_fn();
    println!("[sec12_traits::main] {}", sec12_traits::exercise::display_object());
    println!("[sec12_traits::main] Compare results - {}", sec12_traits::exercise::compare_objects());
}