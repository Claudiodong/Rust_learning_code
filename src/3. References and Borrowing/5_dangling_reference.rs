// if you have a reference to some data, the compiler will ensure that the 
// data will not go out of scope before the reference to the data does.



// problem


fn main() {
    // let reference_to_nothing = dangle();
    println!("{}",no_dangle());
}

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope and is dropped, so its memory goes away.
//   // Danger!



//----------------- solution -----------------

// not reference sign &

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
