// TODO: Implement a function called 'greet' that takes a name parameter
// and prints "Hello, [name]!"
fn greet(name : &str) {
    println!("Hello, {}", name);
    return;
}

// TODO: Implement a function called 'add' that takes two i32 parameters
// and returns their sum
fn add(val1 : i32, val2 : i32) -> i32 {
    return val1 + val2;
}

// TODO: Implement a function called 'is_even' that takes an i32
// and returns true if it's even, false otherwise
fn is_even(val : i32) -> bool {
    if val % 2 == 0 {
        return true;
    }
    return false;
}

// TODO: Implement a function called 'calculate' that takes two i32 parameters
// and returns a tuple of (sum, difference, product)
fn calculate(val1 : i32, val2 : i32) -> (i32, i32, i32) {
    let sum = add(val1, val2);
    let diff : i32 = val1 - val2;
    let product : i32 = val1 * val2;
    (sum, diff, product)
}

fn main() {
    // TODO: Call the greet function with your name
    greet("Alessandro");
    
    // TODO: Call the add function and print the result
    println!("2 + 4 = {}", add(2, 4));
    
    // TODO: Test the is_even function with different numbers
    println!("Is 51 even ? {}", is_even(51));
    
    // TODO: Call calculate and destructure the result
    let (sum, diff, product) = calculate(4, 3);
    println!("Sum : {}, diff : {}, prod : {}", sum, diff, product);
}
