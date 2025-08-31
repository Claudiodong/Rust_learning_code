// to exactly copy the string type (move type) rather than stack them, 

// we could use clone function to help it

fn main(){
    // define the s1
    let s1 = String::from("Hello!");
    
    // assign s1 to s2 by clone
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}"); // should print out extacly the same result
}