use std::io;

fn main() {
    let num1 = 5;
    let num2 = 12;

    println!("There is two numbers, number one and number two.\nThe first number is {num1}. Adding number one and number two we have {}.\nWhat is the value of nubmer two?", num1 + num2);
    
    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("There was an error while reading input");

    println!("\nYour answer is {answer}");
    println!("The right answer is {num2}!");
}
