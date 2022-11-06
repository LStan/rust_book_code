#![allow(unused)]
struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

/*
fn build_user(email:String, username: String) -> User{
    User {
        email: email,
        username: username,
        active: true,
        sing_in_count: 1,
    }
}
*/
fn build_user(email:String, username: String) -> User{
    User {
        email,
        username,
        active: true,
        sing_in_count: 1,
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sing_in_count: 1,
    };

    print!("{}", user1.email);
    
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1 // struct update syntax
    };
    // user1 can't be used here because String 'username' moved to user2

    // --------------------------------

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // -------------------------------

    let subject = AlwaysEqual;
}
