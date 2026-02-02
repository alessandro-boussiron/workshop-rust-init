fn main() {
    println!("=== Ownership Examples ===\n");
    
    // TODO: Observe ownership move
    // Uncomment these lines and see what happens:
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s2);  // This will error! Why?
    
    // TODO: Demonstrate that integers (stack types) are copied, not moved
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);  // This works! Why?
    
    
    
    println!("\n=== Borrowing Examples ===\n");
    
    // TODO: Create a function that takes a reference and returns the length
    fn calculate_length(s: &String) -> usize {
        return s.len();
    }
    let feur : &str = "Feur";
    println!("Name : {}, size : {}", feur, calculate_length(&String::from(feur)));
    
    println!("\n=== Mutable References ===\n");
    
    // TODO: Create a function that modifies a string

    fn append_world(s: &mut String) {
        s.push_str("World");
    }
    // TODO: Create a mutable String and modify it
    let mut s = String::from("Hello");
    println!("{}", s);
    append_world(&mut s);
    println!("{}", s);
    
    fn calculate_length_mut(s: &mut String) -> usize {
        return s.len();
    }

    // TODO: Demonstrate that you cannot have multiple mutable references
    // Uncomment to see the error:
    let mut s = String::from("hello");
    let mut s2 = s.clone();
    let r1 = calculate_length_mut(&mut s);
    let r2 = calculate_length_mut(&mut s2);  // Error!
    println!("{}, {}", r1, r2);
}
    