
// define the struct
struct User {
    // define the type of each key
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


// use the struct in the main loop
fn main() {
    // we call user1 as "instance", 
    // using "mut" in front of user1 to make it mutable, if not -> become immutable
    let mut user1 = User {
        // give the key its value
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // to access 
    println!("User's email is {}", user1.email);

    // change the key value by adding mut user1 
    user1.email = String::from("claudiodong@gmail.com");
    // change the key value by using "string::form"
    println!("New user's email is {}", user1.email);

    }