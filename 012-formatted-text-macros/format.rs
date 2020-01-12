fn main() {
    println!("{} days", 31);
    println!("{0}, {1}, {1}, {0}", "Foo", "Bar");
    println!("{subject} {verb}", subject="foo", verb="jumps");

    println!("base10: {num}, binary: {num:b}", num=2);
    println!("{0:>1$}", 1, 10);

    println!("{:?}", "foo");

    let pi = 3.141592;
    println!("{:.2}", pi)
}
