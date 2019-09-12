fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    let tmp = String::from("hogehogehoge");
    println!("{}",tmp);
    let y :Option<i32> = Option::from(3);

    let y

        : i32 = match y {
        None => 0,
        Some(num) => num,
    };
    x + 1 + y
}