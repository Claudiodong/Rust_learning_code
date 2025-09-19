// now we use the match method with option<T>


// define function with input of x ,
fn add_one(x : Option<i32>) -> Option<i32> { // input and return type are the same
    // the input x is a enum type "Option"
    match x {
        None => None, // if None, become None
        Some(i) => Some(i+1), // Some is a vriant of "Option" enum
    }
}



// main function
fn main(){
    // define a variant
    let five = Some(5);
    // this function should able to add one
    let six = add_one(five);


    // None case
    let none = add_one(None);
    
}