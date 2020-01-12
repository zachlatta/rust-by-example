// Rust by Example on macros doesn't mention this, but it seems like one of the main uses of
// macros is to force compile-time errors
// 
// C macros rely on text expansion. Rust macros expand into the AST, no text stage.
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!()
}
