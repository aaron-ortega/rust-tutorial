// Compute the nth fibonacci number.
// n = n(-1) + n(-2), where
// n(-1) means “the last number before n in the series” and
// n(-2) refers to “the second last one before n in the series.”

// Enter a desired Fibonacci number: 45
// Fn(45): 1134903170

use std::io;

fn main() {
    println!("Enter a desired Fibonacci number: ");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line!");

    let num: i64 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 1, // TODO: learn how to raise Errors
    };

    let result = fibonacci(num);
    println!("Fn({:?}): {:?}", num, result);
}

fn fibonacci(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
