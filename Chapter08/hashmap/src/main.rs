#![allow(unused)]
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    scores.insert(String::from("Blue"), 25); // Overwriting a Value

    // Only Inserting a Value If the Key Has No Value
    scores.entry(String::from("Green")).or_insert(100); // insert
    scores.entry(String::from("Blue")).or_insert(100); // no changes
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
