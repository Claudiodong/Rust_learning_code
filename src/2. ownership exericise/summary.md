// Ownership summary


1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When owner go out of scope, value will be dropped. (To save memory)


Data can be separated into two types, A) Copy type B) Move type
In term of data structure, copy type simply copy the data into two sets, where move type simply move the data from one place to another place.

For example, if we copy a number and store in "E1" and "E2". Same number is stored inside both "E1" and "E2", and there are independent to each other.

However, for the move type, if we copy the value from "E1" to "E2", it result that value move from "E1" to "E2". "E1" will call drop function automatically to free memory, it result "E1" loss the value -> double free problem when call "E1"
--Therefore, solution is to call the "E2" rather than "E1"