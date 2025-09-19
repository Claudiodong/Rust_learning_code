// tuple struct

// define the tuple struct

struct Color(u32, u32, u32);
struct Point(i32, i32, i32);


// main loop
fn main(){
    let black = Color(0,0,0);
    let initial_point = Point(0,0,0);

    // Access through index
    println!("The first index color is {}.", black.0); // should print out "0"


    // Destructing the tuple struct into varible
    let Color(x,y,z) = black;
    println!("After destructing, the x value is:{}.",x);
}