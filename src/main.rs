//import io library from standard library (std)
use std::io;
//Rng does methods that random number generators implement
use rand::Rng;
use std::cmp::Ordering;

//entry into program
fn main() {
    //println! macros
    println!("Guess the Number");

		//rand::thread_rng is local to execution thread, and seeded by OS
    //gen_range function takes a range expression lower..upper or lower..=upper
        let upper: u32 = rand::thread_rng().gen_range(1..=1000);
        let lower: u32 = rand::thread_rng().gen_range(1..upper);
		let secret_number = rand::thread_rng().gen_range(lower..upper);

        println!("The Number is Between {} and {}", lower, upper);
		
    //loop makes an infinite loop for multiple guesses
    loop {
        println!("Please Input Your Guess");

        //let creates an immutable variable
        //mut makes the variable mutable
        //guess is equal to String::new()
        //:: means its a function on the String type
        //guess is now bound to a new empty String
        let mut guess = String::new();
        
        //stdin returns instance of a type that handles input
        io::stdin()
            //calls the read_line on the input handler to append to a String
            //we pass in the guess variable to say where to put the input
            //& means its a reference: lets multiple parts of code access data 
            //references avoid copying the data into memory multiple times 
            //immutable by default so we use mut again
            .read_line(&mut guess)
            //read_line also returns an io::Result value
            //io::Result is an enum (will learn later) with two variants, Ok and Err
            //.expect is a function of Result that lets us catch Err types
            //it will then crash the code and give us an error message
            .expect("Failed to read line");

            //rust lets us shadow our value guess so we can reuse the value with diff type
            //trim removes whitespace
            //parse turns it into a number
            //u32 means unsigned 32 bit integer
            //expect will run if parse doesn't work, like if we tried to parse not a number
            //let guess: u32 = guess.trim().parse().expect("Please type a number");

            //error handling with match
            //trim and parse guess
            let guess: u32 = match guess.trim().parse() {
                //if guess is a number -> return an Ok result and the number
                Ok(num) => num,
                //continue on the loop if Err value and repeat from beginning
                Err(_) => continue,
            };

        //{} holds a value that gets passed in at the end when we know what the value should be
        println!("You Guessed: {}", guess);
        //cmp compares two values, returns variant of ordering enum
		//match is made of arms
		//arms consist of patterns to match against
		//rust takes value of match and compares to arms
		//runs arm that matches
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            //we include a break here so we can escape
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}