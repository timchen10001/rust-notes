use std::io;

fn main() {
    let mut input = String::new();

    some_fn(&mut input);

    io::stdin().read_line(&mut input);

    let mut mars_weight = calculate_weight_on_mars(100.0);

    mars_weight = mars_weight * 1000.0;

    println!("Number: {}", mars_weight);
    // Ok(());
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn some_fn(s: &mut String) {
    println!("{}?", *s);
}