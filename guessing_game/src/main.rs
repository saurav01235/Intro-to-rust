use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    
    let secret_no=rand::thread_rng().gen_range(1,101);
    loop{
        println!("Guess the no!");

    println!("PLease input your guess.");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    println!("You guessed :{}",guess);
    let guess:u32=guess.trim().parse().expect("not a integer");

    match guess.cmp(&secret_no){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too geater"),
        Ordering::Equal => { println!("You win");
        break;
    }
    }
}
}
