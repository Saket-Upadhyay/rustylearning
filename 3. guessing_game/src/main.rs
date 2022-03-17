use std::io;
use rand::Rng;
// We add the line use rand::Rng. The Rng trait defines methods that random number generators implement.
use std::cmp::Ordering;
//The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.

// Start of the main function
fn main()
{
    println!("Guess the number!");

    let _secret_number = rand::thread_rng().gen_range(1..101);

    //We call the rand::thread_rng function that gives us the particular random number generator that we’re going to use: one that is local to the current thread of execution and seeded by the operating system. Then we call the gen_range method on the random number generator. This method is defined by the Rng trait that we brought into scope with the use rand::Rng statement. The gen_range method takes a range expression as an argument and generates a random number in the range. The kind of range expression we’re using here takes the form start..end and is inclusive on the lower bound but exclusive on the upper bound, so we need to specify 1..101 to request a number between 1 and 100. Alternatively, we could pass the range 1..=100, which is equivalent.
loop{
    println!("Input your guess");

    let mut _guess = String::new();
    //this is variable declaration, mut is mutable variable


    //We can also use it like std::io::stdin(); but we already declared that we are using std::io in start. This is similar to delacring the namespace in C++.
    io::stdin()
        .read_line(&mut _guess)
        //The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.
    
        .expect("Faild to read line");

        //Values of the Result type, like values of any type, have methods defined on them. An instance of io::Result has an expect method that you can call. If this instance of io::Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. If this instance of io::Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user’s input.

       let _guess: u32 = match _guess.trim().parse(){
           Ok(num) => num,
           Err(_) => continue,
       };
//If parse is not able to turn the string into a number, it will return an Err value that contains more information about the error. The Err value does not match the Ok(num) pattern in the first match arm, but it does match the Err(_) pattern in the second arm. The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them. So the program will execute the second arm’s code, continue, which tells the program to go to the next iteration of the loop and ask for another guess. So, effectively, the program ignores all errors that parse might encounter!

    // let _guess: u32 = _guess.trim().parse().expect("Input must be integer.");
    //This is a bit complex structure to convert the string number to integer type, we need to convert _guess to integer as _secret_number is integer and the match function below will not work if type are different.
    //The trim method eliminates \n or \r\n

    // The parse method on strings parses a string into some kind of number. Because this method can parse a variety of number types, we need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type. Rust has a few built-in number types; the u32 seen here is an unsigned, 32-bit integer. It’s a good default choice for a small positive number.

    println!("You guessed {}", _guess);

    match _guess.cmp(&_secret_number){
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => {
            println!("You Win!!");
            break;
        },
    }
}
    //A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. Patterns and the match construct are powerful Rust features that let you express a variety of situations your code might encounter and make sure that you handle them all.

}
