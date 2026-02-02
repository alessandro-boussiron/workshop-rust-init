fn main() {
    // TODO: Use if/else to check if a number is positive, negative, or zero
    let number = -5;
    if number > 0 {
        println!("Number positive");
    } else if number == 0 {
        println!("Number null");
    } else {
        println!("Negative number");
    }
    
    // TODO: Use if as an expression to assign a value
    // let result = if condition { value1 } else { value2 };
    let is_positive : bool = if number > 0 {true} else {false};
    println!("Is number positive ? {}", is_positive);
    
    // TODO: Use a loop to calculate the factorial of 5
    // factorial(5) = 5 * 4 * 3 * 2 * 1 = 120
    let mut x = 1;
    let mut result = 1;
    while x <= 5 {
        result *= x;
        x += 1;
    }
    println!("factorial of 5 : {}", result);
    // TODO: Use a while loop to countdown from 5 to 1
    
    let mut countdown = 5;
    while countdown >= 1 {
        println!("{}", countdown);
        countdown -= 1;
    }
    
    // TODO: Use a for loop to iterate from 1 to 5 and print each number
    for i in 0..5 {
        println!("Current number : {}", i);
    }
    
    // TODO: Use a for loop to iterate over an array
    let numbers = [10, 20, 30, 40, 50];
    for elem in numbers.iter() {
        println!("Current number : {}", elem);
    }
    
    // TODO: Use match to classify a number
    // 1 => "one", 2 => "two", 3 => "three", _ => "other"
    let digit = 2;
    match digit {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
    
    // TODO: Use match with ranges
    // 1..=5 => "small", 6..=10 => "medium", _ => "large"
    let range = 7;
        match range {
        1..=5 => println!("small"),
        6..=10 => println!("medium"),
        _ => println!("large"),
    }
}
