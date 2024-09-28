
// Create a function that finds out the average of several numbers ans returns it
fn average(numbers: &[f64]) -> f64 {
    let mut sum = 0.0;
    for number in numbers {
        sum += number;
    }
    // do a loop so that it prints out the numbers
    for number in numbers {
        println!("{}", number);
    }
    sum / numbers.len() as f64
}

fn main() {
    println!("Hello, world!");
    println!("I'm a Rastaban!");
    let item = "mongo";
    let price = 2.5;
    let quantity = 3;
    println!("I bought {} {}s for {} dollars.", quantity, item, price * quantity as f64);
}