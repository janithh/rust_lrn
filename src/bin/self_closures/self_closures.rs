
pub mod test {
    use std::thread;

    pub fn study_fn() {
        define_variants();

        borrow_check();

        thread_check();

        pass_argument_prim_type();

        pass_argument_fn();
    }

    fn define_variants() {
        let a = |x: u32| -> u32 {x * 2};
        let b = |x| {x * 2};
        let c = |x| {x * 2.0};

        println!("[self_closures::test::define_variants] {}", a(10));
        println!("[self_closures::test::define_variants] {}", b(15));
        println!("[self_closures::test::define_variants] {}", c(10.25));

        //let d = |x, y| {x * y}; 

        //println!("[self_closures::test::define_variants] {}", d(127, 127)); 


        //let x: f32 = 12.5;
        //println!("[self_closures::test::define_variants] {}", c(x));

        //let e: u8 = d(5, 6);
        //let e: u32 = b(20);
    }

    fn borrow_check() {
        let mut list = vec![1, 2, 3];

        println!("[self_closures::test::borrow_check] List - {:?}", list);

        let mut a = || -> () { list.push(7) };

        //println!("[self_closures::test::borrow_check] List - {:?}", list);

        a();

        println!("[self_closures::test::borrow_check] List - {:?}", list);
    }

    fn thread_check() {
        let mut list = vec![1, 2, 3];

        println!("[self_closures::test::thread_check] List - {:?}", list); 

        let a = move || -> () { list.push(7) };

        thread::spawn(a).join().unwrap();

        //println!("[self_closures::test::thread_check] List - {:?}", a); 
    }

    fn pass_argument_prim_type() {
        let test = 45;
        let a = |x: i32| -> i32 {x + test};
        println!("[self_closures::test::pass_argument_prim_type] {}", argument_check(a(35)));
    }

    fn argument_check<T>(closure: T) -> T
    {
        closure
    }

    fn pass_argument_fn() {
        let test = 45;
        let a = || { get_test_data(test) };

        println!("[self_closures::test::pass_argument_fn] {:?}", fn_argument_check(a));
        println!("[self_closures::test::pass_argument_fn] {:?}", fn_argument_check(a));
    }

    fn get_test_data(input: u32) -> Vec<u32> {
        let mut data = vec![1, 2, 3];
        data.push(input);
        data
    }

    fn fn_argument_check<F, T>(f: F) -> T 
    where
        F: FnOnce() -> T
    {
        f()
    }
}