use std::io;

fn main() {
    let mut weight = String::new();
    let mut height = String::new();

    println!("Enter your weight in kg:");
    io::stdin().read_line(&mut weight).unwrap();

    println!("Enter your height in meters:");
    io::stdin().read_line(&mut height).unwrap();

    let weight: f64 = weight.trim().parse().unwrap();
    let height: f64 = height.trim().parse().unwrap();

    if height <= 0.0 {
        println!("Height must be greater than zero.");
        return;
    }

    let bmi = weight / (height * height);
    println!("Your BMI is: {:.2}", bmi);

    if bmi < 18.5 {
        println!("Status: Underweight");
    } else if bmi < 25.0 {
        println!("Status: Normal weight");
    } else if bmi < 30.0 {
        println!("Status: Overweight");
    } else {
        println!("Status: Obesity");
    }
}
