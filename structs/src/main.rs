struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
}


fn main() {
    
    let mut user1 = User{
        email: String::from("user1@email.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    println!("username: {}", name);
    user1.username = String::from("user1_updated");

    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("active: {}", user1.active);
    println!("sign_in_count: {}", user1.sign_in_count);


    let user2 = build_user(
        String::from("user2@email.com"), 
        String::from("user2")
    );

    println!("username: {}", user2.username);
    println!("email: {}", user2.email);

    let user3 = User{
        email: String::from("user3@email.com"),
        username: String::from("user3"),
        ..user1
    };

    println!("username: {}", user3.username);
    println!("email: {}", user3.email);


    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle::square(10);

    println!("rect1 is {:#?}", rect1);
    println!("rect2 is {:#?}", rect2);
    println!("rect3 is {:#?}", rect3);

    println!("The area of the rect1 is {} square pixels.", rect1.area());
    println!("The area of the rect2 is {} square pixels.", rect2.area());
    println!("The area of the rect3 is {} square pixels.", rect3.area());

    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1? {}", rect2.can_hold(&rect1));
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
