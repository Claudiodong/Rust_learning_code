1. we enter the keyword "struct" and name the entire struct
2. then we could define the struct as key-value pair as key:value


3.using shorthand function to simply the code by naming the "input" and "key" the same in the function

4. similar, when define the new instance from previous instance, the string type will move it ownership, result loss of user or compile error
e.g., "username: user1.username", in this case, the user1.username, this string will transfer from user 1 to this user, not longer unable to use anymore


5. Tuple struct

Name_of_V (type,type,type);

fn main(){
    let variable = name_of_V(p1,p2,p3);
}

6. Destructing the tuple struct
fn main(){
    let name_of_V(p1,p2,p3) = variable;
}


7. Unit-like struct, is when define the struct as "struct name;", it is a value without data


8. Method, method is a function inside the struct, which allow it to use it without create another function, it able to use the struct value to operate some functionality. by using the "impl" infront of struct_name and adding a function inside it with input parameter "&self"

When you define a method inside an impl, Rust automatically adds the first parameter called self.
But you must decide how you want to pass self:

// &self → an immutable reference to the instance
1.You can read fields  2.You cannot modify them

// &mut self → a mutable reference
1.Lets you change the fields inside the method 

// self (by value)
1.Moves ownership of the whole struct into the method  2.After calling, you can’t use the original instance anymore unless it’s returned


9. Each struct is allowed to have multiple impl blocks. For example, Listing 5-15 is equivalent to the code shown in Listing 5-16, which has each method in its own impl block.


10: Associate function, which not use self in the "()", when call it, we must use "::"


impl Rectangle {
    // Associated function (no self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

let sq = Rectangle::square(10);