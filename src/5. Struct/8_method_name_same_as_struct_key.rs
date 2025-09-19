// now we show a example when the method name is the same as the struct key name

// define the struct
struct Rectangle{
    width: u32,
    height: u32,
}

// method, same name as width, to determine, whether the width is zero or not
impl Rectangle{
    // define the main as name "width"
    fn width(&self) -> bool {
        self.width > 0 // return a bool, if greater than 0, return 1, vice versa
    }
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() { // now is use the method. because it have "()"
            println!("The rectangle has a nonzero width, which is :{}", rect1.width); // now it calling the width key in rect1.
    }
}