use std::io;

fn main() {
    println!("--- BMI Calculator ---");

    // Get weight from user
    let weight = get_input("Enter your weight in kg (e.g., 75.5):");
    
    // Get height from user
    let height = get_input("Enter your height in meters (e.g., 1.75):");

    // Validation to prevent division by zero
    if height <= 0.0 {
        println!("⚠️ Error: Height must be a positive value greater than zero.");
        return;
    }

    // Calculate BMI
    let bmi = weight / (height * height);

    println!("\n------------------------------");
    println!("Your BMI is: {:.2}", bmi);

    // Determine the health category
    let category = if bmi < 18.5 {
        "Underweight"
    } else if bmi < 25.0 {
        "Healthy Weight"
    } else if bmi < 30.0 {
        "Overweight"
    } else {
        "Obesity"
    };

    println!("Health Category: {}", category);
    println!("------------------------------");
}

/// Helper function to read user input and parse it to f64
/// It handles errors gracefully by asking the user to re-enter the value
fn get_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(num) if num > 0.0 => return num,
            Ok(_) => println!("⚠️ Please enter a number greater than zero."),
            Err(_) => println!("⚠️ Error: Please enter a valid number (e.g., 70 or 1.75)."),
        }
    }
}