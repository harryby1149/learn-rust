use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
   new_game();
}

fn new_game() {
    println!("Guess the number!");
    let goal = rand::thread_rng().gen_range(1, 100);
    println!("{:?}", goal);
    prompt_guess(goal);
  
}

fn prompt_guess(goal: i32) {
    println!("Please input your guess:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    handle_guess(goal, guess);
}

fn handle_guess(goal: i32, guess: String) {

    println!("You guessed {}", guess);
    let guess_int: i32 = guess.trim().parse().expect("Guess is Not a Number");
    match guess_int.cmp(&goal) {
        Ordering::Less => {
            println!("Your guess was too low, try again.");
            prompt_guess(goal);
        },
        Ordering::Greater => {
            println!("Your guess was too high, try again.");
            prompt_guess(goal);
        },
        Ordering::Equal => {
            println!("You guessed correctly, Good job! Play again? y/n");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");
            if play_again.trim().eq("y"){
                println!("calling new game");
                return new_game();
            }
        }
    }
}
