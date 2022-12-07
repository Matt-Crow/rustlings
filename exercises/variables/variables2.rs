// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

fn main() {
    // Rust doesn't have an 'int' type. Instead, use i_, where _ is the size in
    // bits. Also, Rust prefers multiline comments using this method 
    let x: i32 = 0x0A;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
