fn main() {
    let target: u128 = 100;


    if target == 0 || target == 1 {
        println!("fibonacci({target}) = {target}");
        return;
    }

    let mut prev: u128 = 0;
    let mut current: u128 = 1;

    for _ in 2..(target+1) {
        let next: u128 = prev + current;
        prev = current;
        current = next;
    }

    println!("fibonacci({target}) = {current}");
}
