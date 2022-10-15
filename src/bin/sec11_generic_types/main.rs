
mod sec11_generic_types;

fn main() {
    /*
     * Section 11
     */
    sec11_generic_types::test::study_fn();

    let box_val_1 = Box::new(12.5);
    let box_val_2 = Box::new(17.5);
    println!("[sec11_generic_types::main] Sum - {}", sec11_generic_types::exercise::sum_boxes(box_val_1, box_val_2));
}