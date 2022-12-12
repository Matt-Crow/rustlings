// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // slices return a subsection of a continuous block of memory
    // note that this does not allocate any new memory, so it's much cleaner
    // than, say for example, splitting a string in C
    // first index is inclusive, second is exclusive
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice);
}
