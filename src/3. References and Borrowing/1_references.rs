// use "&" to reference the value,which will not take any ownership, 
// result the value will not drop after the use.



fn main(){
    let s = String::from("Hello!");

    let len = calculating_length(&s); // reference the "s"

    println!("The world '{s}', have length of {len}");
    // here, since s still have the ownership, we can use it
}

fn calculating_length(s:&String)  -> usize {
    s.len() // return the length of input
}
// here s is out of scope, but we only reference it, never have the ownership, it will not be dropped