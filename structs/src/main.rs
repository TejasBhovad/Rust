#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct Square {
    side: u32,
}
impl Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
    fn can_hold(&self, other: &Square) -> bool {
        self.side > other.side
    }
}
// struct can have multiple implementation methods
impl Square {
    fn set_square(side_len: u32) -> Square {
        Square { side: side_len }
    }
}

fn main() {
    let mut usr = User {
        email: String::from("code.foo.com"),
        username: String::from("Tejas Bhovad"),
        sign_in_count: 2,
        active: false,
    };
    println!("usr data: {:#?}", usr);
    print_user(&usr);

    let name = usr.username;
    println!("Name:{}", name);
    usr.username = String::from("Tejas");

    let _usr_1 = build_user(String::from("foo@gmail.com"), String::from("foo bar"));
    // tuple struct : struct w/o named fields
    struct _Color(i32, i32, i32);
    struct _Point(i32, i32, i32);
    let rect = (12, 3);
    // area_rectangle(rect);
    println!("Area: {}", area_rectangle(rect));

    let sqr = Square { side: 3 };
    let sqr_1 = Square { side: 5 };

    // using imp to call struct methods
    println!("Area of Square: {}", sqr.area());
    println!("Is sqr side greater: {}", sqr.can_hold(&sqr_1));
    println!("Is sqr_1 side greater: {}", sqr_1.can_hold(&sqr));

    // associated function
    let _sqr_2 = Square::set_square(4);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username, //simplified declaration as params name same as attributes
        sign_in_count: 2,
        active: true,
    }
}

fn print_user(usr: &User) {
    println!("Name : {}", usr.username);
    println!("Email : {}", usr.email);
    println!("Sign in times : {}", usr.sign_in_count);
    println!("Active : {}", usr.active);
}
fn area_rectangle(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
