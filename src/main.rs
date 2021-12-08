fn main() {
    let f = 50.0;
    let c = to_celsius(f);

    println!("{}", c);

    let c = 30.0;
    let f = to_fahrenheit(c);

    println!("{}", f);
    println!("{}", fibonaccci(3));
    println!("{}", fibonaccci(4));
    println!("{}", fibonaccci(5));
}

// Convert between Fahrenheit and Celsius
fn to_celsius(n: f32) -> f32 {
    (n - 32.0) * 0.5556
}

fn to_fahrenheit(n: f32) -> f32 {
    (n * 1.8) + 32.0
}
// Generate the nth Fibonacci number
// 1 1 2 3 5 8
fn fibonaccci(n: i32) -> i32 {
    // Base Cases
    if n <= 2 {1}
    else if n == 0{
        0
    }
    else {
        // Recursive Case
        fibonaccci(n - 1) + fibonaccci(n - 2)
    }
}
// Print the lyrics to "The Twelve Days of Christmas"
