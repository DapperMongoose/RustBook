use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);


    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);


    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwrite data in a hash map
    scores.insert(String::from("Blue"), 25);

    // insert only if it doesn't already exist, otherwise keep the old value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
