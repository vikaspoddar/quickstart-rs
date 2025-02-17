/// This module demonstrates ownership and borrowing 

/// Immutable string demo function
/// * ownership
/// * borrowing
/// * immutable borrowing
/// * mutable borrowing
pub fn ownership_demo() {
    // immutable string
    let immutable_string = String::from("hello");
    let immutable_ownership_transfer = immutable_string;
    println!("{}", immutable_ownership_transfer);
}