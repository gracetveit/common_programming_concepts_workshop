fn main() {
    let f = 50.0;
    let c = to_celsius(f);

    println!("{}", c);

    let c = 30.0;
    let f = to_fahrenheit(c);

    println!("{}", f);
}

// Convert between Fahrenheit and Celsius
fn to_celsius(n: f32) -> f32 {
    (n - 32.0) * 0.5556
}

fn to_fahrenheit(n: f32) -> f32 {
    (n * 1.8) + 32.0
}
// Generate the nth Fibonacci number
// Print the lyrics to "The Twelve Days of Christmas"
