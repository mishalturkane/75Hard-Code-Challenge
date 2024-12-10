use std::io;

fn generate_fibonacci(n: usize) -> Vec<u64> {
    let mut fibonacci = vec![0, 1]; // Starting values for the Fibonacci series

    for i in 2..n {
        let next = fibonacci[i - 1] + fibonacci[i - 2];
        fibonacci.push(next);
    }

    fibonacci.truncate(n); // Adjust the length if n < 2
    fibonacci
}

fn main() {
    println!("Enter the number of terms for the Fibonacci series:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let n: usize = match input.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            println!("Please enter a valid positive integer.");
            return;
        }
    };

    let series = generate_fibonacci(n);
    println!("Fibonacci series with {} terms: {:?}", n, series);
}
