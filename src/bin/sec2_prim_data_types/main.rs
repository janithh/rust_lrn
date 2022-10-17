
mod sec2_prim_data_types;

fn main() {
    /*
     * Section 2 
     */
    sec2_prim_data_types::test::study_fn();

    let val_1: u16 = 12;
    let val_2: f32 = 47.5;
    let val_3: f32 = 12.75;
    let average = sec2_prim_data_types::exercise::average(f32::from(val_1), f32::from(val_2), f32::from(val_3));
    println!("[sec2_prim_data_types::main] Average - {average}");
}
