use std::io;

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (9.0/5.0) * celsius + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0/9.0)
}

fn main() {
    println!("Temperature converter program.");
    println!("Enter input temperature with unit letter: [Celsius (c), Fahrenheit (f)]");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");

    let input = input.trim();
    let (val, unit) = input.split_at(input.len() - 1);

    let val: f32 = val.trim().parse().expect("Could not parse input value as number");

    let unit = unit.to_string().to_lowercase();

    println!("val: {}, unit: {}", val, unit);

    let result: f32;
    let mut result_unit = String::new();
    match unit.as_str() {
        "c" => {
            result = celsius_to_fahrenheit(val);
            result_unit = "f".to_string();
        },
        "f" => {
            result = fahrenheit_to_celsius(val);
            result_unit = "c".to_string();
        }
        _ => {
            println!("Invalid unit: {}", unit);
            result = 0.0;
        } 
    }
    
    println!("result is: {:.1}{}", result, result_unit);
}
