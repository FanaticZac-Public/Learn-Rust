// test 1 - struct definition and access

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    println!("{0}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{0}", user1);
}


