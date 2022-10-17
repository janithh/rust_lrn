
mod sec10_structs;
use sec10_structs::test::Rectangle as Rectangle;

fn main() {
    /*
     * Section 10
     */
    sec10_structs::test::study_fn();

    let rectangle = Rectangle::new(2.0, 8.0);
    let scale = 3.0;
    let area = sec10_structs::exercise::scaled_area(&rectangle, scale);
    println!("[sec10_structs::main] Rectangle - {:?}", rectangle);
    println!("[sec10_structs::main] Rectangle area - {area}, Scale - {scale}");
}