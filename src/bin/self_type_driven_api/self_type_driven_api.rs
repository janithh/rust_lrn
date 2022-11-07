
pub mod exercise {
    use std::{time::Duration, thread::sleep, path::Iter};
    use term::Terminal;

    struct Progress<Iter> {
        iter: Iter,
        i: usize,
        bound: Option<usize>,
        delim: (char, char)
    }

    impl<Iter> Progress<Iter> {
        pub fn new(input: Iter) -> Self {
            Progress {
                iter: input,
                i: 0,
                bound: None,
                delim: ('[', ']')
            }
        }
    }

    impl<Iter> Progress<Iter> 
    where Iter: ExactSizeIterator {
        pub fn with_bound(mut self) -> Self {
            self.bound = Some(self.iter.len());
            self
        }
    }

    impl<Iter> Progress<Iter> {
        pub fn with_delims(mut self, delims: (char, char)) -> Self {
            self.delim = delims;
            self
        }
    }

    trait ProgressIterEx: Sized {
        fn progress(self) -> Progress<Self>;
    }

    impl<Iter> ProgressIterEx for Iter {
        fn progress(self) -> Progress<Self> {
            Progress::new(self)
        }
    }

    impl<Iter> Iterator for Progress<Iter> 
    where Iter: Iterator {
        type Item = Iter::Item;
  
        fn next(&mut self) -> Option<Self::Item> {
            let mut t = term::stdout().unwrap();
            t.cursor_up();

            match self.bound {
                Some(bound) => println!("{}{}{}{}", self.delim.0, "*".repeat(self.i), " ".repeat(bound - self.i), self.delim.1),
                None => println!("{}", "*".repeat(self.i))
                
            }

            self.i += 1;
            
            self.iter.next()
        }
    }

    fn calculation(_count: &i32) {
        sleep(Duration::from_secs(1));
    }

    pub fn study_fn() {
        let x = 1.progress();
        let y = "blah".progress();
        let v: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let delimiters = ('<', '>');
        println!("");

        for n in v.iter().progress().with_bound().with_delims(delimiters) {
            calculation(n);
        }

    }
}