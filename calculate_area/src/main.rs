use std::f64::consts::PI;

pub trait CalculateArea {
    fn calculate(&self) -> f64;
}

impl CalculateArea for Triangle {
    fn calculate(&self) -> f64 {
        self.height * self.base * 0.5
    }
}

impl CalculateArea for Circle {
    fn calculate(&self) -> f64 {
        self.radius.powf(2_f64) * PI
    }
}

impl CalculateArea for Rectangle {
    fn calculate(&self) -> f64 {
        self.length * self.width
    }
}

#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64
}

impl Triangle {
    fn new(base: f64, height: f64) -> Triangle {
        Triangle {
            base,
            height
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    width: f64
}

impl Rectangle {
    fn new(length: f64, width: f64) -> Rectangle {
        Rectangle {
            length,
            width
        }
    }
}

#[derive(Debug)]
pub struct Circle {
    radius: f64
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle {
            radius
        }
    }
}

fn area<T: CalculateArea>(t: &T) -> f64 {
    t.calculate()
}

fn main() {
    let circle = Circle::new(2_f64);
    println!("{:?}", (area(&circle)));

    let rectangle = Rectangle::new(2_f64, 3.5_f64);
    println!("{:?}", (area(&rectangle)));

    let triangle = Triangle::new(2_f64, 4.5_f64);
    println!("{:?}", (area(&triangle)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn circle_area() {
        let circle = Circle::new(2_f64);
        assert_eq!((area(&circle)), 2_f64.powf(2_f64) * PI);
    }
}
