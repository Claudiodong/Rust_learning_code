// double free error


fn main() {
    let s1 = String::from("hello");

    let s2 = s1;  // ownership of the heap string moves to s2

    //println!("{s1}, world!"); // error: s1 is no longer valid

    // correct answer should use s2 rather s1, since s1 move to s2
    println!("{s2}, world!"); 

}


