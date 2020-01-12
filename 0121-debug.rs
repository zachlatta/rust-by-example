// traits are like interfaces, derive gets the compiler to automatically implement some
// pre-defined traits, like Debug which makes it easy to print with debug-formatting
#[derive(Debug)]
struct Structure(i32);

fn main() {
    println!("{:?}", Structure(3));
}
