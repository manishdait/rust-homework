fn main() {
    /*
    Ownership Rules:
    - Each value has variable that is call its owner
    - There can only be one owner at a time
    - When owner go out of scop the value gets dropped.
    */

    // x is not valid here because not declare
    {
        let x = String::from("hello"); // x is valid from this point
        // do stuff with x
    } // scope end, so x is not valid after this

    let a = 10;
    let b = a; // copy

    let s1 = String::from("value");
    // let s2 = s1; // move not copy
    let s2 = s1.clone(); // copy

    println!("S1 {}", s1);

    let s = String::from("hello");
    take_ownership(s);

    // println!("S {}", s); // error

    let s1 = give_ownership();
    let s2 = String::from("value");
    let s3 = takes_and_giveback(s2);

    // println!("{}", s2); // s2 is invalid here
    println!("S1 {}, S3 {}", s1, s3);

    // References

    /*
    References Rules:
    - At one time we can have only one mut ref or mutiple unmutable ref
    - Must be valid not dangle
    */

    let s1 = String::from("Hello");
    // let (s2, len) = calculate_str(s);
    let len = calculate_str(&s1);

    println!("S: {}, len: {}", s1, len);

    // only have one mutable ref to the varaible in scope
    let mut s1 = String::from("Hello");
    println!("S1: {}", s1);

    change(&mut s1);
    println!("S1: {}", s1);

    let s = String::from("value");
    let s1 = &s;
    let s2 = &s;

    // can not have mutiple mutable ref and also if unmutable ref exist
    // let s3 = &mut s;

    // Slice

    let s = String::from("Hello World");
    let i = first_word(&s);

    println!("I: {}", i);

    // let hello = &s[0..5];
    let hello = &s[..5];

    // let world = &s[6..11];
    let world = &s[6..];
    println!("S1: {}, S2: {}", hello, world);

    println!("First World: {}", first_word(&s));
}

fn take_ownership(s: String) {
    println!("String {}", s);
}

fn give_ownership() -> String {
    let some_string = String::from("new string");
    some_string
}

fn takes_and_giveback(s: String) -> String {
    s
}

// fn calculate_str(s: String) -> (String, usize) {
//     let len = s.len();
//     (s, len)
// }

// s is refence (borowing) imutable
fn calculate_str(s: &String) -> usize {
    let len = s.len();
    len
}

fn change(s: &mut String) {
    s.push_str("World");
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &b) in bytes.iter().enumerate() {
//         if b == b' ' {
//             return i;
//         }
//     }

//     return s.len();
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i + 1];
        }
    }

    return &s[..];
}
