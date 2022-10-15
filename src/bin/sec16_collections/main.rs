
use std::env;

mod sec16_collections;

fn main() {
    /*
     * Section 16
     */
    sec16_collections::test::study_fn();
    
    let arguments_expected = 2;
    let arguments_num = env::args().len();
    assert_eq!(arguments_expected, arguments_num, "[Main] Invalid arguments : Expected - {arguments_expected}, Received - {arguments_num}");

    let file_path = env::args().nth(1).unwrap();
    sec16_collections::exercise::count_words(&file_path);
}