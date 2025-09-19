// learn how to define a enum, a set of variable

enum IPaddress{ // it able to create a variable that have a set of variable
    V4,// version 4
    V6, // version 6
}


// create instance

fn main(){
   let four = IPaddress::V4; // using "::" to specif which variable
   let six = IPaddress::V6;
}