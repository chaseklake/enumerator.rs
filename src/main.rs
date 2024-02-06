// An example of enumerations, or enums for short

// Since there are only two types of IP addresses, we can create an enum for them:
enum IpAddr {
    V4(u8, u8, u8, u8), // The "(u8...)" tells us we can hold four u8 values
    V6(String), // This acts like an "V6" constructor with a String parameter
}

// We can define functions that can take all types of the enum:
fn route(_ip_kind: &IpAddr) {}

// This is one way to implement a struct that combines the previous enum with a string to hold the address:
//
// struct IPV4 {
//     kind: IpAddrKind,
//     address: String,
// }
//
// But it's better to just add a String type to each variant of the previous enum, which we did

fn main() {
    // Instances of enums
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    // This function takes the enum type, so both variables work here
    route(&four);
    route(&six);

    // An example of calling a function on an enum!
    let m = Message::Write(String::from("hello"));
    m.call();

    // Examples of implementing Option values:
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    
    // Why do this? Because Option<T> and T are different types, so they're incompatible
    let num1: i32 = 5;
    // let sum = num1 + some_number; // ERROR!

    // This means we can eliminate the confusion of whether something is null or not
}

// Enums can do a lot of things that structs can do
enum Message { // Here are four "struct-like" variants grouped into one "Message" enum
    Quit,                       // no associated data
    Move { x: i32, y: i32 },    // two named fields, like a struct
    Write(String),              // a single string
    ChangeColor(i32, i32, i32), // three i32 values
}

// We can even implement methods on enums!
impl Message {
    fn call(&self) {
        // do stuff. We'll learn how to work with enums in the next project using "match"
    }
}

// A very important enum: Option, the "null" enum
// We have it commented out because this already exists in the prelude of every project
// 
// enum Option<T> { // <T> is a generic type parameter, which we'll go over later
//     None,
//     Some(T),
// }
// 
// This enum lets us know if something is there or nothing is there