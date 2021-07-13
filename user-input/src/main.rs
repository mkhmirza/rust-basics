use std::io::{self, Write};

// usage of basic user input
fn main() {
    
    // declaring a new string 
    let mut name = String::new();
    let mut age = String::new();

    print!("Enter Your Name: ");

    // Since your print! statement does not contain nor end in a newline 
    // it will not get flushed
    let _ = io::stdout().flush();
    
    // getting number of bytes, this adds a extra \n in the end of the string 
    let b1 = io::stdin().read_line(&mut name).unwrap();
    
    print!("Enter Your Age: ");
    let _ = io::stdout().flush();
    let b2 = io::stdin().read_line(&mut age).unwrap();
    
    // trimming \n from the input string, 
    // use snake_case for naming variables, naming in other
    // conventions causes a warning
    let trimmed_name = name.trim();
    let trimmed_age = age.trim();

    println!();
    println!("Hello, {}!", trimmed_name);
    println!("You are {} years old.", trimmed_age);   
    // printing int is like this
    // println!("Your Age is: {:?}!", 21);
    // printing bytes each string takes 
    println!("Number of bytes read (name): {}", b1);
    println!("Number of bytes read (age): {}", b2);

}
