use std::io;

fn main() {
    // Read Experience
    println!("Are you experienced? (yes/no):");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("Failed to read line");
    
    // Convert  "yes" to a true/false boolean variable
    let is_experienced = exp_input.trim().to_lowercase() == "yes";

    // Read Age
    println!("Enter your age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: f32 = age_input.trim().parse().expect("Not a valid number");

    //  Evaluate incentive brackets
    if is_experienced && age >= 40.0 {
        println!("Your annual incentive is 1,560,000");
    } 
    else if is_experienced && (30.0..=39.0).contains(&age) {
        println!("Your annual incentive is 1,480,000");
    } 
    else if is_experienced && age < 28.0 {
        println!("Your annual incentive is 1,300,000");
    } 
    else if !is_experienced { // "!" means NOT experienced
        println!("Your annual incentive is 100,000");
    } 
    else {
        println!("No incentive tier matches these criteria.");
    }
}
