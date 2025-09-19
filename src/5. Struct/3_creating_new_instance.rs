// creating new instance(user2) from other instance(user1)

// define the User struct
struct User{
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}


// main loop
fn main(){
    // define the user 1
    let user1 = User{
        active: false,
        email: String::from("123@gmail.com"),
        username: String::from("123_user"),
        sign_in_count: 2,
    };


    // Method 1: define the user 2 based on user 1
    // only update the username, rest of it same as the user 1 

    // do not forget, the string type is a move type, once use in user2, means it transfer from user1 to user2.
    let user2 = User{
        active: user1.active,
        email: user1.email, 
        username: String::from("user2"), // update the username only
        sign_in_count: user1.sign_in_count,
    };

    // Method 2, use "..variable" to define it follow the same as the variable
    let user3 = User{
        username: String::from("user3"),
        // ..user1 // it won't works, because user1.email is transferred to user2 already, missing 
        
        // solution
        email: String::from("user3@gmail.com"),
        ..user1 // now can use, because we do not require the user1.email since we create a new one.

        // However, active and sign_in_count are copy type, it will not affect its ownership.
    }; 


}