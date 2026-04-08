fn main() {
    // let guess: u32 = "Newton".len().try_into().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");

    let another_guess: i32 = "42".parse().expect("Not a number!");

    println!("The guess is: {guess}");

    println!("The another guess is: {another_guess}");

    // Floating-point types

    let x = 2.0; // f64
    println!("The value of x is: {x}");

    let y: f32 = 3.0; // f32
    println!("The value of y is: {y}");

    // Numeric Operations

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

    // Boolean type

    let t = true;
    println!("The value of t is: {t}");

    let f: bool = false;
    println!("The value of f is: {f}");

    // Character type
    let c = 'z';
    println!("The value of c is: {c}");

    let z: char = 'ℤ';
    println!("The value of z is: {z}");

    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    // Compound types
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of tup is: {tup:?}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    // The array type
    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {a:?}");

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("The value of months is: {months:?}");

    let january = months[0];

    println!("The first month is {january}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {a:?}");

    let b = [3; 5];
    println!("The value of b is: {b:?}");

    // Array Element Access
    let c = [1, 2, 3, 4, 5];
    let first = c[0];
    let second = c[1];
    println!("The first element is: {first}");
    println!("The second element is: {second}");
}
