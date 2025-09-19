// multiple methods in one struct


// struct

struct Rectangle{
    width: u32,
    height: u32,
}

// method 1: able to calculate the area
// method 2: determine whether can be hold by other rectangle, with multiple input, by using "other"

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // other: &Rectangle -> able to reference to the struct and key-value pair inside
    fn can_hold(&self, other: &Rectangle) -> bool {  // always use "&" to reference and avoid tansfer of ownership
        self.width > other.width && self.height > other.height
    }
}


// main
fn main(){
    // rect1 
    let rect1 = Rectangle{
        width: 20,
        height: 50,
    };

    let rect2 = Rectangle{
        width: 30,
        height: 20,
    };

    let rect3 = Rectangle{
        width: 10,
        height: 20,
    };


    println!("Can rect 1 hold rect 2 : {}", rect1.can_hold(&rect2));// using &rect2 to avoid transfer of ownership
    println!("Can rect 1 hold rect 3 : {}", rect1.can_hold(&rect3));


    println!("The area of rect 1 is :{} sqaure pixel", rect1.area());
    println!("The area of rect 2 is :{} sqaure pixel", rect2.area());
    println!("The area of rect 3 is :{} sqaure pixel", rect3.area());

}