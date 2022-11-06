use std::collections::HashMap;

fn main() {
    let vec = vec![1, 1, 3, 4, 5, 4, 2, 4, 6, 0];
    assert_eq!(mean(&vec), 3.0);
    assert_eq!(median(&vec), 3.5);
    assert_eq!(mode(&vec), 4);

    assert_eq!(convert_to_pig_latin("first"), "irst-fay");
    assert_eq!(convert_to_pig_latin("apple"), "apple-hay");
}

//fn mean(vec: &Vec<i32>) -> f64 {
fn mean(vec: &[i32]) -> f64 {
    vec.iter().sum::<i32>() as f64 / vec.len() as f64
}

fn median(vec: &[i32]) -> f64 {
    let mut sorted = vec.to_vec();
    sorted.sort_unstable();
    if vec.len() % 2 == 0 {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) as f64 / 2.0
    } else {
        sorted[sorted.len() / 2] as f64
    }
}

fn mode(vec: &[i32]) -> i32 {
    let mut num_count = HashMap::new();
    for &num in vec {
        //let count = num_count.entry(num).or_insert(0);
        //*count += 1;
        *num_count.entry(num).or_insert(0) += 1;
    }
    let mut max_value = 0;
    let mut mode = 0;
    println!("{:?}", num_count);

    for (key, value) in num_count {
        if value > max_value {
            max_value = value;
            mode = key;
        }
    }
    mode
}

fn convert_to_pig_latin(word: &str) -> String {
    
/*
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first_letter = word.chars().next().unwrap();
    //let first_letter = word.chars().nth(0).unwrap();
    let other: String = word.chars().skip(1).collect();
*/
    let vowels = ["a", "e", "i", "o", "u"];
    let (first_letter, other) = word.split_at(1);
    if vowels.contains(&first_letter) {
        format!("{word}-hay")
    } else {
        format!("{other}-{first_letter}ay")
    }
}
