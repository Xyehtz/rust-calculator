use std::io;

fn main() {
    println!("Hello, please select one of the following options\n----------------------------");
    println!("1. Addition\n2. Subtraction\n3. Multiplication\n4. Division");

    loop {
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input...");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number...");
                continue;
            }
        };

        let mut first_number = String::new();
        let mut second_number = String::new();

        println!("Please enter the first number");
        io::stdin()
            .read_line(&mut first_number)
            .expect("Failed to read input...");

        println!("Please enter the second number");
        io::stdin()
            .read_line(&mut second_number)
            .expect("Failed to read input...");

        let first_number: f64 = match first_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number...");
                continue;
            }
        };
        let second_number: f64 = match second_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number...");
                continue;
            }
        };

        if option == 1 {
            println!("Result: {}", add_numbers(&first_number, &second_number));
        } else if option == 2 {
            println!("Result: {}", subtract_numbers(&first_number, &second_number));
        } else if option == 3 {
            println!("Result: {}", multiply_numbers(&first_number, &second_number));
        } else if option == 4 {
            println!("Result: {}", divide_numbers(&first_number, &second_number));
        } else {
            println!("Please enter a valid option...");
        }
    }
}

fn add_numbers(n1: &f64, n2: &f64) -> f64 {
    n1 + n2
}

fn subtract_numbers(n1: &f64, n2: &f64) -> f64 {
    n1 - n2
}

fn multiply_numbers(n1: &f64, n2: &f64) -> f64 {
    n1 * n2
}

fn divide_numbers(n1: &f64, n2: &f64) -> f64 {
    n1 / n2
}
