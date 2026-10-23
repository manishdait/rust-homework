struct User {
    username: String,
    emial: String,
    sign_in_count: u64,
    active: bool,
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// like static
impl  Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    } 
}

fn main() {
    let mut user1 = User{
        emial: String::from("test.com"),
        username: String::from("vongo"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("voldemort");

    let user2 = build_user(String::from("test.com"), String::from("kim"));
    let user3 = User{username: String::from("kate"), ..user2};


    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 10,
        height: 12,
    };

    println!("rect: {:#?}", rect);
    println!("{}", rect.area());

    let square = Rectangle::square(10);
}

fn build_user(emial: String, username: String) -> User {
    User {
        emial,
        username,
        active: true,
        sign_in_count: 1,
    }
}



// fn area(rect: &Rectangle) -> u32 {
//     rect.height * rect.width
// }
