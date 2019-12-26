/* If a type is not in the standard library of Rust, we have to import it first */

// Return a random value supporting the [`Standard`] distribution
use rand::Rng;
// An ordering where a compared value is less / equal / bigger than another
use std::cmp::Ordering;
// Used in this program to obtain user input
use std::io;

// In Rust, the starting point of every program is in the main function
fn main() {
    println!("Welcome to the Guess the number game!");
    // Generate a random number between 1 and 100 (low = including / high = excluding)
    let sec_num = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", sec_num);

    // A loop that asks the user for a number, until the secret_number is found.
    loop {
        println!("Please input your number: ");
        // Create a mutable variable to store the user input
        // ::new() indicates that new is an associated function of the String type
        let mut guess = String::new();
        //  ::stdin() is a type that represents a handle to the standard input for the terminal
        io::stdin()
            /*
            read_line calls the read_line method to get input from user and stores it in the
            passed argument (&mut guess)
            & indicates that it is a reference type
            */
            .read_line(&mut guess)
            /*
            Expect is an instance of io::Result. It can have two values:
            Ok => It will take that value and return it
            Err => It will cause the program to crash and display an error message
            If we don't call .expect, the program will compile, but with a warning.
            Rust tells us then, that the program hasn't used the Result value from read_line and
            it is indicating that the program hasn't handled a possible error
            */
            .expect("Failed to read line");

        /*
        We create a variable called guess of integer type u32 that shadows the previous value of guess
        .trim() => eliminates all white space at the beginning and the end
        .parse() => parses the string into a number. If it successfully converts the string to a number
        it will return the Ok variant of Result, otherwise the user has entered a non-numeric value and
        the return will be an Err with the message, that the user has to enter a number and should try again
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("No characters allowed, please type a number");
                /*
                Part of the Loop. As long as the user enters a non-numeric value, it will keep asking
                until the user enters a number
                */
                continue;
            }
        };

        println!("You guessed the number {}", guess);

        // Based on the secret_number, this will return the correct Ordering value
        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                // If the correct secret_value has been guessed, the program stops
                break;
            }
        }
    }
}
