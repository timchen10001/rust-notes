use std::io;

fn main() {
    println!("Enter Your Weight (kg): ");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    println!("Input: {}", weight);
    
    let mars_weight = calculate_weight_on_mars(weight);

    println!("Number: {}", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}