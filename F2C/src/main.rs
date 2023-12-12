use std::io;

fn main() {
    // Prompt the user for input
    println!("Enter the temperature in Fahrenheit:");

    // Read user input
    let mut fahrenheit_input = String::new();
    io::stdin()
        .read_line(&mut fahrenheit_input)
        .expect("Failed to read line");

    // Parse the input into a f64 (floating-point number)
    let fahrenheit: Result<f64, _> = fahrenheit_input.trim().parse();

    match fahrenheit {
        Ok(fahrenheit) => {
            // Convert Fahrenheit to Celsius using the conversion formula
            let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

            // Display the result with a max of 2 decimals
            println!("{} Fahrenheit is {:.2} Celsius", fahrenheit, celsius);
        }
        Err(_) => {
            eprintln!("Invalid input. Please enter a valid number.");
        }
    }
}
