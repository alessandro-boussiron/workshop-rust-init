fn main() {
    // TODO: Create an immutable variable 'x' with value 5
    // Try to change it - what happens?
    let x = 5;
    println!("x = {}", x);
    
    // TODO: Create a mutable variable 'y' with value 10
    // Then change it to 20 and print it
    let mut y = 10;
    y = 20;
    println!("Y = {}", y);
    
    // TODO: Create a variable with explicit type annotation
    // let age: u32 = ...;
    let age: u32 = 17;
    println!("age = {}", age);
    
    // TODO: Create boolean and char variables
    let feur: bool = false;
    let val: char = 'c';
    
    // TODO: Demonstrate shadowing
    // Create a variable 'spaces' as a string "   "
    // Then shadow it with the length of the string
    let spaces = "   ";
    let length = spaces.len();
    
    // TODO: Work with different numeric types
    // Create an i32, u32, f64 and perform some operations
    let val1 : i32 = 20;
    let val2 : u32 = 30;
    let val3 : f64 = 10.0;
    println!("Result : {}", val1 as u32 + val2 + val3 as u32);
}
