use std::fs::File;
use std::io::Read;

pub fn read_file_content<'a>(path: &'a str) -> std::io::Result<String> {
    let mut content = String::new();
    match File::open(path) {
        Ok(mut f) => f.read_to_string(&mut content).and_then(|_| Ok(content)),
        Err(e) => { println!("cannot open file: {}", e); Err(e) }
    }
}

macro_rules! dump {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {}, "),*), $($a),*);
    }
}
