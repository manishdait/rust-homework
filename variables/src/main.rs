fn main() {
    // Integer
    let a: i32 = 16;
    let b = 0xa;
    let c = 0b1100;
    let d = 0o14;

    // Floats
    let e = 34.3;
    let f: f32 = 34.3;

    // Boolean
    let g: bool = true;
    let h = false;

    // Character
    let i = 'a';
    let j: char = 'e';

    // Tuple
    let tup = ("Jhon", 34);

    // Destructure
    let (name, age) = tup;
    
    // point
    let age = tup.1;

    // Array
    let arr = [1, 2, 4];
    let ele = arr[0];
    let new_array = [0; 8]; // [0, 0, 0, 0, 0, 0, 0, 0]

    const TIMEOUT: u32 = 34_000;
    // const mut TIMEOUT: u32 = 34;

    let sum = my_fn(1, 2);
    println!("{sum}");

    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count;
        }
    };

    count = 0;
    while count < 10 {
        count += 1;
    }

    println!("{}", count);

    println!("Count: {}", result);

    let arr = [10, 20, 30];
    for ele in arr.iter() {
        println!("Element: {}", ele)
    }

    for i in 1..4 {
        println!("{}!", i);
    }

    println!("Fibo {}", fibo(7));
}

fn my_fn(x: i32, y: i32) -> i32 {
    println!("Value of x: {}", x);
    println!("Value of y: {}", y);
    x + y
}


fn fibo(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    return fibo(n-1) + fibo(n-2);
}
