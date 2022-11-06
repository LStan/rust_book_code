#![allow(unused_variables)]

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let mut x = 5;
    let y = Box::new(x);
    let z = &mut x;

    *z += 1;

    assert_eq!(6, *z);
    assert_eq!(6, x);
    assert_eq!(5, *y);
}