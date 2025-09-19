// this file including the use of "match"


//define a enum: including different types of coin
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}
// each coin have different, we can use match method to determine their value

// define the function
fn value_of_coin(coin : Coin){ // input parameter is "Coin", name is "coin"
    match coin{ // match the input parameter name "coin" but not coin
      Coin::Penny => 1,  // using :: to target the variable type inside the enum "Coin", 
      // using "=>" to give value
      Coin::Nickel => 5,
      Coin::Dime => 10, // called arm
      Coin::Quarter => 25,

    }
}

// function mathch with long arm code
fn lucky_coin(coin : Coin){ // input parameter is "Coin", name is "coin"
    match coin{ // match the input parameter name "coin" but not coin
      Coin::Penny => {
        println!("Lucky coin");
        1   // ensure still print out the value, long arm code could use {}
      },
    }
}