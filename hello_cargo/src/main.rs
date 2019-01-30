extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");          // 数を当ててごらん

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");   // ほら、予想を入力してね

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to readline.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => {num},
            Err(_) => {continue},
        };


        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("small!") }
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
            Ordering::Greater => { println!("too big"); }
        }
    }
}