use std::io; // Standard In/Out Library

fn main() {
    println!("Guess the number !");

    println!("Please input your guess : ");

    let mut guess = String::new(); // Mut has to be used to make variables mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess is : {guess}");
}
