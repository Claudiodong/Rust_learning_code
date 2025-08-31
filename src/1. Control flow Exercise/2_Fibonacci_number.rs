// Generate the nth Fibonacci number.
//  0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233
// formula n_{i} = n_{i-1} + n_{i-2} only vaild from i >= 3

fn main(){
    let i: i32 = 4; // must always larger than 3

    if i == 0 {
        println!("The {}th Fibonacci number is 0", i);
    }
    else if i == 1{
        println!("The {}st Fibonacci number is 1", i);
    }
    else{
       println!("The Fibonacci number is {}", fib_num(i));
    }
}


// function, only activate when i is equal or greater than 3
fn fib_num(i: i32) -> i64 {
    let mut num1 = 0; // able to change 
    let mut num2 = 1; // able to change
   for _index in 1..i{ // counting from 1
    let num3 = num1 + num2; // n_{i} = n_{i-1} + n_{i-2}
    // update the number
    num1 = num2;
    num2 = num3;
   }
   num2 // output the num2 since num2 = num3 once finish the loop and function
}