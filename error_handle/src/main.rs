use core::panic;
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    a();
    enum _Result<T, E> {
        Ok(T),
        Err(E),
    }
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem while creating a file:{:?}", e),
            },
            other_error => {
                panic!("Other Error:{:?}", other_error)
            }
        },
    };

    let _f = File::open("hello_world.txt").unwrap();
}

fn a() {
    b();
}

fn b() {
    c(21)
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass 22!")
    }
}
