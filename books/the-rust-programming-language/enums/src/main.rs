// enum IpAddrKind {
//     V4,
//     V6,
// }
// fn main() {
//     let four = IpAddrKind::V4;
//     let size = IpAddrKind::V6;
// }

// enum IpAddrKind {
//    V4,
//    V6,
// }

// struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
// }

// fn main() {
//    let home = IpAddr {
//        kind: IpAddrKind::V4,
//        address: String::from("127.0.0.1")
//    };

//    let loopback = IpAddr {
//        kind: IpAddrKind::V6,
//        address: String::from("::1"),
//    };
// }

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// enum Message {
//     Quite,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {}
// }

// struct QuiteMessage;
// struct MoveMessgae {
//     x: i32,
//     y: i32,
// }

// struct WriteMessage(String);

// struct ChangeColorMessage(i32, i32, i32);

// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// fn main() {
//     let some_number = Some(5);
//     let some_char = Some('c');
//     let absent_number: Option<i32> = None;
// }

// fn main() {
//     let x = 5;
//     let y = Some(5);
//     let sum = x + y;
// }

// enum Coint {
//     Penny,
//     Nickle,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coint: Coint) -> i32 {
//     match coint {
//        Coint::Penny => 1,
//        Coint::Nickle => 5,
//        Coint::Dime => 10,
//        Coint::Quarter => 25,
//     }
// }

// fn value_in_cents(coint: Coint) -> u8 {
//     match coint {
//         Coint::Penny => {
//             println!("Lucky Penny");
//             1
//         },
//         Coint::Nickle => 5,
//         Coint::Dime => 10,
//         Coint::Quarter => 25,
//     }
// }

// enum Coint {
//     Penny,
//     Nickle,
//     Dime,
//     Quarter(UsState),
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// fn value_in_cents(coint: Coint) -> u8 {
//     match coint {
//         Coint::Penny => 1,
//         Coint::Nickle => 5,
//         Coint::Dime => 10,
//         Coint::Quarter(state) => {
//             println!("State quarter from {state:?}.");
//             25
//         }
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//        None => None,
//        Some(a) => Some(a+1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}
// fn main() {
//     let dice = 8;
//     match dice {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other)
//     }
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn reroll() {}
// fn main() {
//     let dice = 8;
//     match dice {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => reroll()
//     }
// }

// fn main() {
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {max}"),
//         _ => (),
//     }
// }

// fn main() {
//     let config_max = Some(3u8);

//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {max}");
//     }
// }

enum Coint {
    Penny,
    Dime,
    Nickle,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

// fn main() {
//     let mut count = 0;
//     let coint = Coint::Quarter(UsState::Alabama);

//     match coint {
//         Coint::Quarter(state) => println!("State quarter from {state:?}"),
//         _ => count += 1,
//     }

//     let mut count = 0;
//     let coint = Coint::Quarter(UsState::Alaska);
//     if let Coint::Quarter(state) = coint {
//         println!("State coint is from {state:?}");
//     } else {
//         count += 1;
//     }
// }

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

// fn describe_state_quarter(coint: Coint) -> Option<String> {
//     if let Coint::Quarter(state) = coint {
//         if state.existed_in(1900) {
//             Some(format!("{state:?} is pretty old, for America!"))
//         } else {
//             Some(format!("{state:?} is relatively new."))
//         }
//     } else {
//         None
//     }
// }

// fn describe_state_quarter(coint: Coint) -> Option<String> {
//     let state = if let Coint::Quarter(state) = coint {
//         state
//     } else {
//         return None;
//     };

//     if state.existed_in(1990) {
//          Some(format!("{state:?} is pretty old, for America!"))
//     } else {
//         Some(format!("{state:?} is relatively new."))
//     }
// }

fn describe_state_quarter(coint: Coint) -> Option<String> {
    let Coint::Quarter(state) = coint else {
        return None;
    };

    if state.existed_in(1990) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let coint = Coint::Quarter(UsState::Alaska);
    println!("{:?}", describe_state_quarter(coint));
}
