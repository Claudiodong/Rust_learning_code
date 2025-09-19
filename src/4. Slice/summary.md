1. Rather than a reference to the entire String, hello is a reference to a portion of the String, specified in the extra [0..5] bit. We create slices using a range within brackets by specifying [starting_index..ending_index], 

where starting_index is the first position in the slice and ending_index is one more than the last position in the slice. Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index. 

So, in the case of let world = &s[6..11];, world would be a slice that contains a pointer to the byte at index 6 of s with a length value of 5.

code://
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
//


// some syntax

1. let slice = &s[0..2]; // is same as  &s[..2] since start from 0

2. let slice = &s[3..len];// if len is the last one, we could use "let slice = &s[3..];"

3. let slice = &s[..]; // to represent whole word

//
&str is an immutable reference.
//\


////////////////////////////////////////////////////////////////////////////////////////////
1. The Problem

Suppose you write a function first_word to find the first word in a String.

Without slices, you’d return the index of where the word ends.

But indices can get out of sync if the string changes (e.g., calling .clear() makes your saved index meaningless).

This creates a logic bug that the compiler can’t catch.

//
let s = String::from("hello world");
let hello = &s[0..5]; // "hello"
let world = &s[6..11]; // "world"
// Internally, a slice stores:
a pointer to the first element
a length
//

use as function input and return value ** &str ** to avoid the function take the ownership.