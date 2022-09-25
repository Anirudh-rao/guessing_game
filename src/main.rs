//To Import Libraries we use "Use" keyword
use std::io; //importing Std::io - standard input and output
use rand::Rng; //importing the Rand Crate
use std::cmp::Ordering; //To Compare two Variables


fn main() {
    println!("Guess the Number");

    // rand::thread_rng function that gives us the particular random number generator
    //gen_range method on the random number generator
    //The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds
    let secret_number  =  rand::thread_rng().gen_range(1..=100);

    loop{
    println!("Please input your guess");
    //declaring a mutable vairable to store guess
    //In rust, Variables are immutable by default
    //The String::new returns new instance of the varaible Guess everytime
    let mut guess = String::new();
    
    //To read the guess
    //We use readline function to pass the pointer of guess
    //We also expect the failed result and hence the error message
    io::stdin().read_line(&mut guess).expect("Failed to readline");

    let guess: u32 = match guess.trim().parse() {
        //error function takes in two variables
        //pass meaning accepted variable and err - else fail
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed:{guess}");

    //compares between Secret_number and guess and returns Ordering variable
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("To Greate"),
        Ordering::Equal => {
            println!("You Win");
            break;
        }
    }
}

}
