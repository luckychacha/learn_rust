use std::f64::consts::PI;

pub trait CalculateArea {
    fn calculate(&self) -> f64;
}

impl CalculateArea for Triangle {
    fn calculate(&self) -> f64 {
        self.length * self.width * 0.5
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

struct Triangle {
    length: f64,
    width: f64
}

struct Rectangle {
    length: f64,
    width: f64
}

pub struct Circle {
    radius: f64
}

#[derive(Debug)]
struct OtherShape {
    length: f64,
    width: f64
}

fn area<T: CalculateArea>(t: &T) -> f64 {
    t.calculate()
}

fn main() {
    let circle = Circle {radius: 2_f64};
    println!("{:?}", (area(&circle)));

    let other_shape = OtherShape {
        length: 2_f64,
        width: 3_f64
    };
    println!("{:?}", other_shape);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn circle_area() {
        let circle = Circle {radius: 2_f64};
        assert_eq!((area(&circle)), 2_f64.powf(2_f64) * PI);
    }
}
