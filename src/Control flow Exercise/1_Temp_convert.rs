// convert the temperature between Fahrenheit and Celsius.


// function
fn F_to_C(Fahre_input: i32) -> f64 {
    let celsius = (Fahre_input -32) as f64 * 5.0/9.0;
    celsius // output it straight away
}


// main loop
fn main(){
    let Fahre = 100;
    println!("The Celsius temperature is {}", F_to_C(Fahre));
}