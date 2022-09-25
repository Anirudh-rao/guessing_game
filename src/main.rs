//To Import Libraries we use "Use" keyword
use std::io; //importing Std::io - standard input and output

fn main() {
    println!("Guess the Number");
    println!("Please input your guess");
    //declaring a mutable vairable to store guess
    //In rust, Variables are immutable by default
    //The String::new returns new instance of the varaible Guess everytime
    let mut  guess =  String::new();
    
    //To read the guess
    //We use readline function to pass the pointer of guess
    //We also expect the failed result and hence the error message
    io::stdin().read_line(&mut guess).expect("Failed to readline");

    println!("You guessed:{guess}");
}
