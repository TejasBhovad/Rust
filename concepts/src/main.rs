fn main() {
    let mut x = 4;
    println!("x: {}", x);
    x = 8;
    println!("x: {}", x);
    const SIZE: u32 = 1_00_000;
    println!("SIZE: {}", SIZE);

    let decimal = 45.09;
    println!("decimal: {}", decimal);

    // tuples
    let tup = ("foo", 23.4, 3);
    let (str, dec, num) = tup;
    println!("tuple[str]: {}", str);
    println!("tuple[dec]: {}", dec);
    println!("tuple[int]: {}", num);
    println!("tuple[str] using dot: {}", tup.0);
    println!("tuple[dec] using dot: {}", tup.1);
    println!("tuple[int] using dot: {}", tup.2);

    let ar = [1, 3, 4, 5];
    println!("ar[1]: {}", ar[1]);

    // declaring array with 5 elements wit all values 0
    let ar = [0; 5];
    println!("ar[4]: {}", ar[4]);

    // function call
    my_function();
    my_function_params(1, 4);

    // function with return value
    let result = add(3, 8);
    println!("Result: {}", result);

    // function with conditional control flow
    conditions();
}

fn my_function() {
    println!("new function called");
}

fn my_function_params(x: i32, y: i32) {
    println!("x: {}", x);
    println!("y: {}", y);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn conditions() {
    let condition = true;
    let number = if condition { 8 } else { 9 };
    println!("Number: {}", number);

    loop {
        println!("* ");
        break;
    }

    // Loops can return a value
    let mut ctr = 0;
    let result = loop {
        ctr += 1;
        if ctr == 10 {
            break ctr;
        }
    };
    println!("result after loop: {}", result);

    // while loop
    let mut num = 3;
    while num != 0 {
        println!("{}", num);
        num -= 1;
    }

    // for loop
    let a = [23, 45, 2, 45, 6, 1];

    for element in a.iter() {
        println!("value: {}", element);
    }

    // array loop with index
    for (index, &element) in a.iter().enumerate() {
        println!("ar[{}]: {}", index, element);
    }

    // loop from 1->3
    for element in 1..3 {
        println!("value: {}", element);
    }
}
