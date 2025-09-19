// method is a type of function, it inclduing extra "self", and it is defined inside the function

#[derive(debug)]

// struct
struct Rectangle {
    width: u32,
    height: u32,
}

// method, define a method inside the struct, allow instance to use
impl Rectangle {
    // define the function
    fn area(&self) -> u32 {// define the retuen
           self.width * self.height // because we return this value, no ";"    
    }
}


// main loop
fn main(){
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!("The area of the Rectangle is {} square pixels", rect1.area);
}