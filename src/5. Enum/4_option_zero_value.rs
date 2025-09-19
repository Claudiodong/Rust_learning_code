// using  option<T> to define the zero value that is 
// currently absent or no exist now, but might have in the future time


// define enum
enum option<T>{
    None,   // is variant type of option<T>
    Some(T),
}

// <T> means that the Some variant of the Option enum can hold one piece of data of any type, and that each concrete 
// type that gets used in place of T makes the overall Option<T> type a different type.

fn main(){
    let some_number = Some(5); // -> become Some(i32)
    let some_char = Some('e'); // -> become Some(char)

    let absent_number: Option<i32> = None; // -> define absent_number have i32 tppe, but right now is None.
    // Option<T> and T is different type //
    // So, Option(i32) and i32 are completely different type
    // cannot adding, substract
}