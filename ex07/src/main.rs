// TODO: Define a Direction enum with variants: North, South, East, West

enum Direction {
    North,
    South,
    East,
    West,
}

struct Move {x : i32, y : i32}

// TODO: Define a Message enum with these variants:
// - Quit (no data)
// - Move { x: i32, y: i32 } (struct-like)
// - Write(String) (tuple-like)
// - ChangeColor(i32, i32, i32) (tuple-like with RGB)

enum Message {
    Quit,
    Move {x : i32, y : i32},
}

// TODO: Implement a function that takes a Direction and returns a string
// fn direction_to_string(dir: Direction) -> &'static str

fn direction_to_string(dir: Direction) -> &'static str {
    match dir {
        Direction::North => "North",
        Direction::East => "East",
        Direction::South => "South",
        Direction::West => "West",
    }
}

// TODO: Implement a function that processes a Message
// fn process_message(msg: Message)

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Returning"),
        Message::Move {x, y} => println!("Moving to : {} {}", x, y),
    }
}

// TODO: Implement a function that uses Option<T>
// fn divide(a: f64, b: f64) -> Option<f64>
// Returns Some(result) if b != 0, None otherwise

fn divide(a : f64, b : f64) -> Option<f64> {
    if b == 0.0 {
        return None;
    }
    Some(a / b)
}

// TODO: Implement a function that adds one to an Option<i32>
// fn plus_one(x: Option<i32>) -> Option<i32>

fn plus_one(x : Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("=== Direction Enum ===");
    // TODO: Create Direction instances and print them
    let north = Direction::North;
    let south = Direction::South;
    
    println!("Direction: {} | {}", direction_to_string(north), direction_to_string(south));
    
    println!("\n=== Option Enum ===");
    // TODO: Test the divide function with different inputs
        match divide(10.0, 2.0) {
        Some(result) => println!("10.0 / 2.0 = {}", result),
        None => println!("Cannot divide by zero"),
    }
    
    match divide(10.0, 0.0) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot divide by zero!"),
    }
    
    // Using plus_one
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("five + 1 = {:?}", six);
    println!("none + 1 = {:?}", none);
}
