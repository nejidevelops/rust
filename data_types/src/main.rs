fn main() {
    // let guess: u32 = "Newton".len().try_into().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");

    let another_guess: i32 = "42".parse().expect("Not a number!");

    println!("The guess is: {guess}");

    println!("The another guess is: {another_guess}");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // Addition
    let sum = 5 + 10;
    println!("The sum is: {sum}");

    // Subtraction
    let difference = 95.5 - 4.3;
    println!("The difference is: {difference}");

    // Multiplication
    let product = 4 * 30;
    println!("The product is: {product}");

    // Division
    let quotient = 5.0 / 2.0;
    println!("The quotient is: {quotient}");

    // Remainder
    let remainder = 5 % 2;
    println!("The remainder is: {remainder}");
}
