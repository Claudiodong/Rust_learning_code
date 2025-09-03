// print out struct all together

#[derive(Debug)] // adding this able to ignore some error

struct Rectangle {
    height: u32,
    width: u32,
}

fn main(){
    let rect1 = Rectangle{
        height: 20,
        width:30,
    };

    println!("rect1 is {rect1:?}");// able to print out the rect1 all information during debug.

}



