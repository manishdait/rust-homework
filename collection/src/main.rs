fn main() {
    let mut vec: Vec<u32> = Vec::new();
    vec.push(1);
    vec.push(2);

    let v2 = vec![1, 2, 3, 4];
    let third = &v2[2];
    println!("Third: {}", third);

    let safe_value = v2.get(20);
    match safe_value {
        Some(x) => println!("Third: {}", x),
        _ => println!("No value present")
    };
}
