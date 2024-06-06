use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut initial_guess = String::new(); // mutable
     io::stdin()
         .read_line(&mut initial_guess)
         .expect("Failed to read line");
    println!("You guessed : {}", initial_guess);

    some_example_input_output();

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    println!("The secret number is : {}", secret_number);

    //compare_guess_to_secret(guess, secret_number);
    compare_guess_to_secret_continual(initial_guess, secret_number);
}

fn compare_guess_to_secret_continual(mut guess: String, secret_number: u32) {
    loop {
        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input. Please input a number.");
                guess.clear();
                io::stdin().read_line(&mut guess).expect("Failed to read line");
                continue;
            }
        };

        println!("You guessed: {}", guess_number);

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("Input your guess.");
        guess.clear();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    }
}

fn compare_guess_to_secret(guess: String, secret_number: u32) {
    // let mut guess = String::now();
    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

fn some_example_input_output() {
    let apples = 5; // immutable
    println!("apples = {}", apples);

    let x = 5; let y = 10;
    println!("x = {} and y = {}", x, y);
    println!("Input continual number by space!");
    let a = input();
    println!("{:?}\n", a);
}

fn input_guess()->String {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new(); // mutable
    io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");
    println!("You guessed : {}", guess);
    return guess;
}

fn input()->Vec<String> {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}

