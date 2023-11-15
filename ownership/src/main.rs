fn main() {
    // --- OWNERSHIP RULES ---
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can be only one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped

    // scope
    {
        // no variables can be accessed
        let _str = "hello";
        let _growable_str = String::from("hello");
        // str and growableStr can be accessed
    }
    // end of scope

    let x = 5;
    let _y = x;
    println!("{} was the original number", x);
    // here the value of x gets copied not moved thus even x can be accessed
    // most data types other than Strings get copied

    let s1 = String::from("foo");
    let _s2 = s1;
    // move not shalllow copy (both pointers not pointing to same memory) (default)

    // println!("{} was the word", s1); //<-- this statement is throws an error as s1 vales are moved to s2 and not cloned

    let s3 = String::from("foo_2");
    let _s4 = s3.clone();
    // cloning means the pointer will point to a cloned copy of data and not to og address
    // clone() operations are expensive

    let sf = String::from("function string");
    random_function(sf);
    // println!("String: {}", sf); throws error as passing arg is considered as moving values of strings in Rust

    // References: points to where pointer of target variable is pointing (can have more than one reference)
    let len_str = String::from("foo");
    let len = calculate_length(&len_str);
    println!("Length of {}: {}", len_str, len);

    // mutable reference (only one mutable reference is possible)
    let mut mut_string = String::from("Hello ");
    // change using reference variable changes both values
    change(&mut mut_string);
    println!("mut_string: {}", mut_string);
    // we cant mutable reference if immutable reference already exist

    // String Slices
    let mut s = String::from("hello world");
    let _hello = &s[0..5]; // or [..5]
    let _world = &s[6..11]; // o [6..]
    let _full_s = &s[..];

    let word = first_word(&s);
}

fn random_function(s: String) {
    println!("String: {}", s);
}

fn calculate_length(s: &String) -> usize {
    let len = s.len();
    // references to string cant be modified
    len
}

fn change(str: &mut String) {
    // append string
    str.push_str("world");
    println!("str: {}", str);
}
// NOTE: after a variable is no longer used its scope ends meaning if there are no more uses
// below, they don't exist, this can be useful declaring mutable and immutable references

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
