// similarly, copy and move type value can pass to the function


// For move type, the variable cannot be used after the function, because the drop function is call 
// to free the memory

// for the copy type, the variable can still be used after the function,




// main

fn main(){

    // define the move type 
    let s = String::from("Move type!"); // introduce the s into scope
    take_ownership(s);  // s value move into the function
    //-------------------- after this line, s cannot be used any more, since it is free from memory---------------------//
    // take_ownership(s);  // calling it again will result error
    // println!("s");

    // define the copy type
    let c: i32 = 32; // introduce the c into scope
    copy_type(c);
    
    //------------- However, copy type will not be affected since it is not free from memory
    copy_type(c); // can be call multiple times.

}


// function for move type
fn take_ownership(string_input:String){
    println!("{string_input}");// if s still can be used, should able to print out "Move type"
}

// function for copy type
fn copy_type(integer_input:i32){
    println!("{integer_input}");
}
    