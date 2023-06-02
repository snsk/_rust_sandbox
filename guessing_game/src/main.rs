use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let try_limit_count = 5;
    let mut try_count = 0;

    loop {

        if try_count >= try_limit_count{
            println!("GAME OVER");
            break;
        }

        println!("Plese input your guess");
        let mut guess = String::new(); //mut mean the "mutable"
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };//変数をシャドーイングする, エラー処理をmatch式で行う
        //ResultはOk, Errの列挙子を持つ列挙型

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        try_count += 1;
        println!("You have {} more tries left", 5 - try_count);
    }
}
