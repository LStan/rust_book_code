#![allow(unused)]
fn main() {
    let s1 = String::from("hello");
    let len = calculate_lenght(&s1);

    println!("The lenght of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // s.push_str("aaa");
    // println!("{}, {}, and {}", r1, r2, r3);

    //-------------------
   
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_lenght(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}