// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

fn main() {
    // an array of 100 42's
    // could also do 
    //   let a = [1, 2, 3, 4, 5];
    //   let a: [u8; 5] = [...];
    let a = [42; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
