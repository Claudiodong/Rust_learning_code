// using impl to define method inside the enum


// define the enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


// define the method
impl Message{
    fn call(&self){
        // method main body
    }
}


// main 
fn main(){
    let m = Message::Write(String::from("Hello"));
    // string::from -> create a string
    // &str is borrowing a string from other -> take ownership

    // using the method
    m.call();
}
