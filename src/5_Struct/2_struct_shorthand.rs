// rather than repeat the word in the struct

// we could easier to address the issues by put down same name.

// define the struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


// define the function
fn build_user(email: String, username: String) -> User { // return value is "User"
    User {
        active: true,
        username, // we do not need to write down the username and email, since there are the same as input
        email,
        sign_in_count: 1,
    }
}


fn main() {
    // define the user
   let user1 = build_user(String::from("123@gmail.com"), String::from("123"));
   // printout the value to confirm
   println!("email address is:{}, username is:{}", user1.email, user1.username);
}