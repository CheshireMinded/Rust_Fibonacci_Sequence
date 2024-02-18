use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::collections::HashMap;
use std::io;

fn main() {
    loop {
        println!("Select an option:");
        println!("1. Calculate factorial (n!)");
        println!("2. Fibonacci calculation");
        println!("3. Calculate π to n terms");
        println!("0. Exit");

        let choice = read_option();
        match choice {
            1 => calculate_factorial(),
            2 => calculate_fibonacci(),
            3 => calculate_pi_series(),
            0 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option. Please enter a number between 0 and 3."),
        }
    }
}

fn calculate_factorial() {
    println!("Enter n for factorial calculation:");
    let n = read_u64();
    println!("{}! = {}", n, factorial(n).to_str_radix(10));
}

fn factorial(n: u64) -> BigInt {
    let mut result = BigInt::one();
    for i in 1..=n {
        result *= i;
    }
    result
}

fn calculate_fibonacci() {
    println!("Enter n for Fibonacci calculation:");
    let n = read_u64();
    let mut fib = Fibonacci::new();
    println!("Fibonacci({}) = {}", n, fib.calculate(n));
}

struct Fibonacci {
    memo: HashMap<u64, u64>,
}

impl Fibonacci {
    fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }

    fn calculate(&mut self, n: u64) -> u64 {
        if n < 2 {
            return n;
        }
        if let Some(&value) = self.memo.get(&n) {
            return value;
        }
        let result = self.calculate(n - 1) + self.calculate(n - 2);
        self.memo.insert(n, result);
        result
    }
}

fn calculate_pi_series() {
    println!("Enter the number of terms for π calculation:");
    let terms = read_u64();
    println!("π approximated to {} terms is {}", terms, calculate_pi(terms));
}

fn calculate_pi(terms: u64) -> f64 {
    let mut sum = 0.0;
    for k in 0..terms {
        let term = ((-1.0_f64).powi(k as i32) * 4.0) / (2.0 * k as f64 + 1.0);
        sum += term;
    }
    sum
}

fn read_option() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u32>().unwrap_or_else(|_| {
        println!("Please enter a valid number.");
        0
    })
}

fn read_u64() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u64>().unwrap_or_else(|_| {
        println!("Please enter a valid number.");
        0
    })
}
