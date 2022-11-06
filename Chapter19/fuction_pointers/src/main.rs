#![allow(unused)]

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    assert_eq!(answer, 12);

    // fuctions instead of  closures
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings_closure: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings_function: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    assert_eq!(list_of_strings_closure, list_of_strings_function);

    // enum varian as an initializer function
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

enum Status {
    Value(u32),
    Stop,
}
