mod ascii;
mod colors;
mod shapes;
use ascii::BANNER;
use colors::{CYAN, RESET};
use shapes::{Area, Circle, Parallelogram, Rectangle, Square, Triangle};

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("{CYAN}{}{RESET}", BANNER);

    println!("[1] Triangle\n[2] Rectangle\n[3] Square\n[4] Circle\n[5] Parallelogram");
    let shape_type = shapes::input("Enter the shape type: ");

    match shape_type as i32 {
        1 => {
            let triangle: Triangle = Triangle::construct();
            println!("Area of Triangle is: {}", triangle.area());
        }
        2 => {
            let rectangle: Rectangle = Rectangle::construct();
            println!("Area of Rectangle is: {}", rectangle.area());
        }
        3 => {
            let square: Square = Square::construct();
            println!("Area of Square is: {}", square.area());
        }
        4 => {
            let circle: Circle = Circle::construct();
            println!("Area of Circle is: {}", circle.area());
        }
        5 => {
            let parallelogram: Parallelogram = Parallelogram::construct();
            println!("Area of Parallelogram is: {}", parallelogram.area());
        }
        _ => println!("Not a valid shape type."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn area_of_triangle_works() {
        let triangle = Triangle {
            base: 102.0,
            height: 165.0,
        };
        assert_eq!(triangle.area(), 8_415.0);
    }

    #[test]
    fn area_of_rectangle_works() {
        let rect = Rectangle {
            length: 19.0,
            breadth: 26.0,
        };
        assert_eq!(rect.area(), 494.0);
    }
    //
    #[test]
    fn area_of_circle_works() {
        let circle = Circle { radius: 40.0 };
        assert_eq!(circle.area().ceil(), 5027.0);
    }
}
