// TODO: Define a Rectangle struct with width and height fields (both u32)
struct Rectangle {
    width: u32,
    height: u32,
}

// TODO: Implement methods for Rectangle
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {width, height}
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        (2 * self.width) + (2 * self.height)
    }
    fn is_square(&self) -> bool {
        self.height == self.width
    }
}


// TODO: Define a User struct with username (String), email (String), and age (u32)

struct User {
    username: String,
    email: String,
    age: u32,
}

// TODO: Implement a constructor for User
impl User {
    fn new(username: String, email: String, age: u32) -> User {
        User {username, email, age}
    }
    fn display_info(&self){
        println!("username : {} | email : {} | age : {}", self.username, self.email, self.age);
    }
}


// TODO: Define a tuple struct Point with two f64 values

struct Coord(f64, f64);

// TODO: Implement a method for Point to calculate distance from origin
impl Coord{
    fn new(x : f64, y : f64) -> Coord {
        Coord(x, y)
    }
    fn distance_from_origin(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
}

fn main() {
    // TODO: Create a Rectangle instance and use its methods
    let rect = Rectangle::new(500, 250);
    println!("{} {} {}", rect.area(), rect.perimeter(), rect.is_square());
    
    // TODO: Create a User instance and display info
    
    let user = User::new(String::from("Tellvex"), String::from("test"), 17);
    user.display_info();
    
    // TODO: Create a Point and calculate distance from origin
    
    let point = Coord::new(80.0, 10.0);
    println!("{}", point.distance_from_origin());
}
