fn main() {
    //guess in this context needs a data type otherwise error
    let guess: i32 = "42".parse().expect("Not a number!");
    println!("The guess is: {guess}");
    let num1 = 5;
    let num2 = 10;
    //this will overflow bc num3 is unsigned
    //let num3: u32 = num1 - num2;
    let num3 = num1 - num2;
    println!("num1 - num2 = {num3}");

    // addition
    let sum = 5 + 10;
    p
    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
