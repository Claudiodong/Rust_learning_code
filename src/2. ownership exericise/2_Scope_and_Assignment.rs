// when double assign two string values to same variable
// the first assign value will be free by drop function in rust automatically to free memory


fn main(){
    // define the s with mutability
    let mut s = String::from("hello");

    // re-assign the s variable
    s = String::from("ahoy");

    // should print out "ahoy, world" rather than "hello, world"
    println!("{s}, world!");

}