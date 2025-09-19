// bind method to extract enum variant

// Match arms must return the same type -> if preivous are string type, must all string type return

#[derive(Debug)]

// define first enum
enum Name{
    Claudio,
    Aurola,
}


// define the second enum
enum Gender{ // must insert the previous enum in this enum, to ensure current enum "Gender" carry data "Name"
    boy(Name),
    girl(Name),
    other,
}


// define match function
fn tell_name(_gender : Gender) {
    match _gender {
        Gender::boy(name) => { // once define the enum and its variant, give the variant a new input "name"
           // print out the boy's name
           println!("The boy's name is {name:?} !"); // ":?" is used for debug method
        }

        Gender::girl(name) => {
            println!("The girl's name is {name:?} !");
        }

        // must be same type of return
        Gender::other => {
            println!("Nothing !");
        },
    }
}

// main 

fn main(){

    // call the function with boy
    tell_name(Gender::boy(Name::Claudio));// insert enum with another enum

    // call the function with boy
    tell_name(Gender::girl(Name::Aurola));// insert enum with another enum

    tell_name(Gender::other);
}