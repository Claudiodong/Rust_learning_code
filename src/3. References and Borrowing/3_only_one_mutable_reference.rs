// we only can create mutable reference once for one variable in a scope

//-------------------- problem -------------------------------------
fn main(){
    let mut s = String::from("hello");

    let r1 = &mut s; // first
    let r2 = &mut s; // second

    println!("{r1}, {r2}");
    // result error, cannot create a second reference
}


//----------------------------- solution ------------------------------//

// create a new scope, borrow once in each scope

// fn main(){
//     // first scope
//     let mut s = String::from("hello");

//     let r1 = &mut s; // first
//     println!("r1 is: {r1}");

//     // second scope
//     {
//          let r2 = &mut s; // second
//          println!("r2 is: {r2}");
//     }
// }