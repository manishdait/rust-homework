struct Alien {
    username: String,
    email: String,
    signin_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // equivalent to static method
    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Tuple Struct

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let bob = Alien {
        username: String::from("Bob"),
        email: String::from("bob@alien.com"),
        signin_count: 0,
        active: true,
    };
    let username = bob.username;

    println!("Username: {}", username);

    // mutate value
    let mut kim = Alien {
        username: String::from("Kim"),
        email: String::from("kim@alien.com"),
        signin_count: 0,
        active: true,
    };
    println!("Username: {}", kim.username);

    kim.username = String::from("Jhon");
    println!("Username: {}", kim.username);

    let tim = build_alien(String::from("tim@test.com"), String::from("Tim"));

    let rob = Alien {
        username: String::from("Rob"),
        email: String::from("rob@test.com"),
        ..tim
    };

    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    // println!("Area: {}", area(&rect));
    println!("Rectangle: {:?}", rect);
    println!("Rectangle: {:#?}", rect);
    println!("Area: {}", rect.area());

    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 40,
    };

    println!("R1 canHolde R2: {}", rect1.can_hold(&rect2));
    println!("R2 canHolde R1: {}", rect2.can_hold(&rect1));

    let square = Rectangle::square(10);
    println!("Square: {:#?}", square);
}

fn build_alien(email: String, username: String) -> Alien {
    // Alien {
    //     username: username,
    //     email: email,
    //     signin_count: 0,
    //     active: true,
    // };

    // since the field are same as struct field.
    Alien {
        username,
        email,
        signin_count: 0,
        active: true,
    }
}

// fn area(rect: &Rectangle) -> i32 {
//     rect.height * rect.width
// }
