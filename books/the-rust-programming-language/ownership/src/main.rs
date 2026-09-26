// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{s1}, world!");
// }

// fn main() {
//     let mut s = String::from("hello");
//     s = String::from("ahoy");

//     println!("{s}, world!");
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {s1}, s2 = {s2}");
// }

// fn main() {
//     let s = String::from("Hello, world!");

//     take_ownership(s);

//     println!("{s}");

//     let x = 5;

//     make_copy(x);

//     println!("{x}")
// }

// fn take_ownership(some_string: String) {
//     println!("{some_string}");
// }

// fn make_copy(some_integer: u32) {
//     println!("{some_integer}");
// }

// fn main() {
//     let s1 = gives_ownership();
//     let s2 = String::from("hello");
//     let s3 = takes_and_gives_back(s2);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn main() {
//     let s1 = String::from("hello");
//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is: {len}");
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let l = s.len();
//     (s, l)
// }

// fn main() {
//     let s = String::from("hello");
//     let length = calculate_length(&s);

//     println!("The length of '{s}' is: {length}");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let s1 = String::from("hello");

//     change(&s1);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world!");
// }

fn main() {
    let mut s1 = String::from("hello");
    change(&mut s1);

    println!("{s1}");
}

fn change(some_string: &mut String) {
   some_string.push_str(", world!"); 
}