


1. reference a value but not take ownership by using "&"

2. cannot modify the reference unless adding "mut"

3. only allow "mutable" reference once in a scope, multiple mutable reference in multiple scope

4. allow multiple immutable reference in a scope, allow one mutable reference after the immutable references in a scope

5. must define the location of memory without reference, because reference need a memory location. can not return a reference in the function 


6. // immutable reference

fn print_name(name: &String) {
    println!("{}", name);
}


7. // mutable reference

fn change_name(name: &mut String) {
    name.push_str(" Jr.");
}
