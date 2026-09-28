// TODO: structs book section

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("some@example.com"),
//         sign_in_count: 1,
//     };
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("some@example.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotherexample@example.com");
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user1 = build_user(
//         String::from("some@example.com"),
//         String::from("someusername123"),
//     );

//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };

//     println!("username: {}", user1.username);
// }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let color = Color(0, 0, 0);
    let point = Point(0, 0, 0);
}
