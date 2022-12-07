// variables4.rs
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a hint.

fn main() {
    // variables are immutable by default, so need to explicitely declare them
    // as mutable
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
