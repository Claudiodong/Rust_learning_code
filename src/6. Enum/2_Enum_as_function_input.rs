// make it as the input of function

// define the enum
enum IPaddress{
    V4,
    V6,
}

// we could use struct to embedde the enum inside the struct
struct IpAdree{
    kind: IPaddress,
    address: String,
}


// define a function, make the enum as input parameter
fn IProute(ip_type: IPaddress){ // ip_type is name of input variable, IPaddress is data type

}


// main
fn main(){
    IProute(IPaddress::V4); // make it as input, but specify the type of address
    IProute(IPaddress::V6);


    // define the instance by using struct

    let home = IpAdree{
        kind: IPaddress::V4, // define the kind variable type as V4
        address: String::from("127.01.01.01")
    };
    
    let company = IpAdree{
        kind: IPaddress::V6, // define the kind variable type as V6
        address: String::from("000.00.00.00")
    };
}