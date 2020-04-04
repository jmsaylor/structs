fn main() {
    let mut user1 = User {
        email: String::from("yadayada@gmail.com"),
        password: String::from("1234"),
        login_count: 4,
        active: true,
    };
    let mut user2 = build_user(String::from("frank@gmail.com"), String::from("4321"));

    let franks_brother = User { ..user2 };
    println!("{}", franks_brother.password);
}

struct User {
    email: String,
    password: String,
    login_count: u32,
    active: bool,
}

fn build_user(email: String, password: String) -> User {
    User {
        email,
        password,
        login_count: 0,
        active: false,
    }
}
