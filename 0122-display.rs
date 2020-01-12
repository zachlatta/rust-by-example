use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lol - {}", self.0)
    }
}

fn main() {
    let s = Structure(3);

    println!("{}", s);
}
