fn main() {
    // let guess: u32 = "Newton".len().try_into().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");

    let another_guess: i32 = "42".parse().expect("Not a number!");

    println!("The guess is: {guess}");

    println!("The another guess is: {another_guess}");
}
