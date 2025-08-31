// borrowing = creating a reference


fn main() {
    let s = String::from("hello");

    change(&s); // result error, since we cannot change the reference
}

// this function try to change or modify the reference "s".
fn change(some_string: &String) {
    some_string.push_str(", world");
}



// solution adding mut, mention it is mutable

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s); // change the reference "s" that borrow
//     println!("{s}"); // print out "s" which already change
// }

// // this function try to change or modify the reference "s".
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }