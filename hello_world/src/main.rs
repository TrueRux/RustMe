trait Shape {
    fn area(&self) -> f64;
}

struct Cycle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Cycle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * &self.radius * &self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        &self.width * &self.height
    }
}

#[derive(Debug)]
struct BbA {
    width: u32,
    height: u32,
}

fn main() {
    let cycle = Cycle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.5,
        height: 3.6,
    };

    println!("Hello, world! {}", cycle.area());
    println!("Hello, world! {}", rectangle.area());

    let testa = BbA {
        width: 30,
        height: 60,
    };

    println!("{:#?}", testa);
}
