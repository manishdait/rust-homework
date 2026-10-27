fn main() {
    // let x = 5; default imutable
    let mut x = 5;
    println!("x: {}", x);

    x = 6;
    println!("x {}", x);

    const COUNT: u32 = 10_000;

    // shadowing
    let y = 10;
    println!("y: {}", y);

    let y = 30;
    println!("y: {}", y);

    let y = "ten";
    println!("y: {}", y);

    // Scalar Datatype

    // integer (default i32)
    let a = 10_000; // decimal
    let b = 0xFF; // hex
    let c = 0o10; // octal
    let d = 0b1010; // binary
    let e = b'A'; // u8 only

    // decimals (default f64)
    let f = 20.32323;
    let g: f32 = 20.43;

    // bool
    let h = true;
    let i = false;

    // char
    let j = 'A';
    let k = '😍';

    // Compound DataType

    // tuple
    let tup = ("Bob", 20);
    let (name, age) = tup; // destructuring
    println!("Name: {}, Age: {}", name, age);

    let age = tup.1; // dot operation
    println!("Age: {}", age);

    // array
    let arr = [1, 2, 3, 4];
    let ele = arr[0];
    println!("Arrray Ele {}", ele);

    let arr2 = [0; 4]; // [0, 0, 0, 0]

    // Function
    let sum = my_fn(10, 10);
    println!("Sum {}", sum);

    // Conditions
    let number = 10;
    if number > 10 {
        println!("greater than 10");
    } else if number < 10 {
        println!("less than 10");
    } else {
        println!("equal to 10");
    }

    let condition = true;
    let value = if condition { 20 } else { 30 };

    // Loops

    // loop
    let mut count = 0;
    // loop {
    //     count += 1;
    //     if count == 10 {
    //         break;
    //     }
    // }

    let result = loop {
        count += 1;
        if count == 10 {
            break count;
        }
    };

    println!("Counter {}", result);

    // while

    let mut count = 3;
    while count > 0 {
        println!("Inside while");
        count -= 1;
    }

    // for

    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("I: {}", i);
    }

    for num in 1..4 {
        println!("#{}", num);
    }
}

fn my_fn(x: i32, y: i32) -> i32 {
    println!("x: {}", x);
    println!("y: {}", y);
    println!("My Funtion");

    // return x + y;
    x + y
}
