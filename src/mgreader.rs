use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub mod mgreader {
    use std::{
        fs::File,
        io::{self, prelude::*},
        rc::Rc,
    };

    pub struct BufReader {
        pub reader: io::BufReader<File>,
        buf: Rc<String>,
    }
    
    fn new_buf() -> Rc<String> {
        Rc::new(String::with_capacity(1024)) // Tweakable capacity
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            let buf = new_buf();

            Ok(Self { reader, buf })
        }
    }

    impl Iterator for BufReader {
        type Item = io::Result<Rc<String>>;

        fn next(&mut self) -> Option<Self::Item> {
            let buf = match Rc::get_mut(&mut self.buf) {
                Some(buf) => {
                    buf.clear();
                    buf
                }
                None => {
                    self.buf = new_buf();
                    Rc::make_mut(&mut self.buf)
                }
            };

            self.reader
                .read_line(buf)
                .map(|u| if u == 0 { None } else { Some(Rc::clone(&self.buf)) })
                .transpose()
        }
    }
}
