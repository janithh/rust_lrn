
pub mod test {
    /*
     * Shuttle struct
     */
    struct Shuttle<'a> {
        name: &'a str
    }

    impl<'a, 'b> Shuttle<'a> {
        pub fn new(shuttle: &'a str) -> Shuttle<'a>{
            Shuttle {
                name: shuttle
            }
        }

        pub fn send_transmission(&self, msg: &'b str) -> &'b str {
            println!("[Shuttle::test::send_transmission] Shattle - {}, Transmitting message - {}", self.name, msg);
            msg
        }
    }

    pub fn study_fn() {
        borrow_check();

        lifetime_annotation();
    }

    fn borrow_check() {
        //Borrow checker
        let propellant;
        {
            let rp = String::from("RP-1");
            //propellant = &rp;
            propellant = rp;
            println!("[sec13_lifetimes::test::study_fn] Propellant - {}", propellant);
        }
        println!("[sec13_lifetimes::test::study_fn] Propellant - {}", propellant);
    }

    fn lifetime_annotation() {
        //Lifetime annotation syntax
        let prop_1 = String::from("RP-1");
        let result;
        {
            let prop_2 = String::from("LNG");
            result = best_fuel(&prop_1, &prop_2);
            println!("[sec13_lifetimes::test::study_fn] Best Fuel - {}", result);
        }
        //println!("[sec13_lifetimes::study_fn] Best Fuel - {}", result); /* This will not work due to prop_2 life time is short */
  
        //lifetime ellision rules
        let first_word = get_first_word("My name is Janith");
        println!("[sec13_lifetimes::test::study_fn] First word  - {}", first_word);

        //Lifetime annotations
        let hubble = Shuttle::new("Hubble");
        let message = "Greetings from Hubble !!!";
        let sender = hubble.send_transmission(message);

        println!("[sec13_lifetimes::test::study_fn] Sender - {sender}");

        //static lifetime
    }

    fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        if x.len() > y.len() {x} else {x} 
    } 

    fn get_first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (index, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..index];
            }
        }

        &s
    }
}