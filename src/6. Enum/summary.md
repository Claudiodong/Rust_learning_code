1. enmu, always make a variable as a set of variables.

e.g., IP address have two type method, IPv4 and IPv6, make the IP adress as a set of variable of two encrypt technology.



2. enum can be used as the input of function, make the function able to accept a set of variable 

function synatx
{

    fn name_of_function(input_name : input_type){}
}


3. could use the struct, then embedd the enum inside the struct. allow it for multiple variable type

4. A quicker way to define the instance without creata struct as following"
    
enum IpAddr{
    V4(String),
    V6(String),
    }

fn main(){
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

to insert the string inside the enum without define the struct, create a instance from the enum.


5. different type of variable can be used inside the enum, string, number, struct, other enum and so on.


6. enum could be defined with different type variable: {
   enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// it able to store no data value
// similar struct type
// string
// tuple as well

}


7. we are able to define method in the enum as well by using impl


8. using "Option<T>" to define some enum that is currently absent or not useful, but might useful and have value in the future.

Option is a enum type inside the Rust, where "Some" is a variant of "Option" enum


9. "Match" -> allow to compare a value to series of patterns, use it inside the value => 5_match.rs

10. Using match to bind -> take out the value from enum variant => 6_bind.rs
need to add data inside the enum, ensure we could add it in the future function
// Match arms must return the same type


11.  using option <T> with match method => 7_match_with_option.rs
// must have "None" inside the match method when input is "Option" type


12. match require to fill all possible inputs and it is exhaust, instead, we could use "other" to use other undefine value, or "_" to ignore other value. => 8_match_all_case.rs