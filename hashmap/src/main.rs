use std::collections::HashMap;
fn main() {
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 30);
    scores.insert(String::from("Red"), 40);
    scores.insert(String::from("Red"), 90);

    // println!("{}", blue); throws error

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    let text = "Hello World World Hey";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
