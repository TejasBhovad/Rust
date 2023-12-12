fn main() {
    let _a = [1, 2, 3];
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let mut vector = vec![1, 2, 3, 4];
    // can cause Runtime errors
    let third = &vector[2];
    // vector.push(20); cant mutate when reference is taken
    println!("Third element {}", third);

    // to prevent crashing and handle error
    match vector.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // looping thu vectors
    for i in &vector {
        println!("{} ", i);
    }

    // mutating looping
    for i in &mut vector {
        *i += 20;
    }

    // modifies og values
    for i in &vector {
        println!("{} ", i);
    }
}
