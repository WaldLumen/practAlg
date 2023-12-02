use std::io;
use std::f64::consts::PI;

pub fn maklorens_row_for(mut n:f64, iterations:i32) {
    let mut input = String::new();

    println!("Enter x: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let x: f64 = match input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let mut summa = 0.0;


    for _ in 1..iterations {
        let gamma = 2.0_f64.powf(-2.0 * n) + 4.0_f64.powf(-2.0 * n) + 6.0_f64.powf(-2.0 * n) + 8.0_f64.powf(-2.0 * n) + 10.0_f64.powf(-2.0 * n);
        let b = (-1.0_f64).powf(n + 2.0) * factorial(2.0 * (2.0 * n - 1.0)) / (2.0_f64 * PI).powf(2.0 * n) * gamma;
        let result = ((b * factorial(2.0 * n - 1.0) / factorial(2.0 * n)) / factorial(2.0 * n)) * x.powf(2.0 * n - 1.0);
        print!("{} ", result);
        n += 1.0;
        summa += result;
    }
    println!("\n Summa = {}", { summa }); // Add a newline after printing results
}
    pub fn maklorens_row_while (mut n: f64, accuracy: f64) {
        let mut input = String::new();

        println!("Enter x: ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let x: f64 = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Invalid input. Please enter a valid number.");
                return;
            }
        };

        let mut summa = 0.0;
        let mut current_term: f64 = 1.0;
        let mut term_number = 1;

        while current_term.abs() > accuracy {
            let gamma = 2.0_f64.powf(-2.0 * n) + 4.0_f64.powf(-2.0 * n) + 6.0_f64.powf(-2.0 * n) + 8.0_f64.powf(-2.0 * n) + 10.0_f64.powf(-2.0 * n);
            let b = (-1.0_f64).powf(n + 2.0) * factorial(2.0 * (2.0 * n - 1.0)) / (2.0_f64 * PI).powf(2.0 * n) * gamma;
            let result = ((b * factorial(2.0 * n - 1.0) / factorial(2.0 * n)) / factorial(2.0 * n)) * x.powf(2.0 * n - 1.0);
            print!("{} ", result);
            n += 1.0;
            summa += result;
            current_term = result;
            term_number += 1;
        }

        println!("\n Summa = {}", summa); // Add a newline after printing results
    }

pub fn factorial(num: f64) -> f64 {
    (1..=num as u64).product::<u64>() as f64
}
