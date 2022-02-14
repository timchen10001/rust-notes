use std::io;

fn main() {
    let mut input = String::new();

    let s1 = &input;
    let s2 = &input;

    print!("{}{}", s1, s2);

    some_fn(&mut input);
    
    io::stdin().read_line(&mut input);

    borrow_string(&mut input);
    own_string(input);
    
    let mut mars_weight = calculate_weight_on_mars(100.0);

    mars_weight = mars_weight * 1000.0;

    println!("Number: {}", mars_weight);
    // Ok(());
}

fn borrow_string(s: &String) {
    print!("{}", s);
}

fn own_string(s: String) {
    print!("{}", s);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn some_fn(s: &mut String) {
    s.push_str("pushed");
    println!("{}", *s);
}