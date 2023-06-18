fn input(prompt: &str) -> f32 {
    use std::io::{stdin, stdout, Write};
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut choice = String::new();
    stdin()
        .read_line(&mut choice)
        .expect("Failed to get user input");
    let choice: f32 = choice.trim().parse().expect("Failed to parse to f32");
    choice
}

// Defining Structs
pub struct Triangle {
    pub base: f32,
    pub height: f32,
}

pub struct Rectangle {
    pub length: f32,
    pub breadth: f32,
}

pub struct Square {
    pub length: f32,
}

pub struct Parallelogram {
    pub base: f32,
    pub height: f32,
}

pub struct Circle {
    pub radius: f32,
}

// Implementing Structs
impl Triangle {
    pub fn construct() -> Self {
        let base = input("Please enter base of triangle: ");
        let height = input("Please enter height of triangle: ");
        Self { base, height }
    }
}

impl Rectangle {
    pub fn construct() -> Self {
        let length = input("Please enter length of Rectangle: ");
        let breadth = input("Please enter breadth of Rectangle: ");
        Self { length, breadth }
    }
}

impl Square {
    pub fn construct() -> Self {
        let length = input("Please enter the length of Square: ");
        Self { length }
    }
}

impl Parallelogram {
    pub fn construct() -> Self {
        let base = input("Please enter the base of Parallelogram: ");
        let height = input("Please enter the height of Parallelogram: ");
        Self { base, height }
    }
}

impl Circle {
    pub fn construct() -> Self {
        let radius = input("Please enter the radious of Circle: ");
        Self { radius }
    }
}

// Implementing Traits for Structs
pub trait Area {
    fn area(&self) -> f32;
}

impl Area for Triangle {
    fn area(&self) -> f32 {
        (self.base * self.height) / 2.0
    }
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.length * self.breadth
    }
}

impl Area for Square {
    fn area(&self) -> f32 {
        self.length * self.length
    }
}

impl Area for Parallelogram {
    fn area(&self) -> f32 {
        self.base * self.height
    }
}

impl Area for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}
