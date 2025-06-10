use std::f64::consts::PI;
fn main() {}
struct Triangle {
    sides_lens: [f64; 3],
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

trait Shape {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
}

impl Shape for Triangle {
    fn get_area(&self) -> f64 {
        let [a, b, c] = self.sides_lens;
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }

    fn get_perimeter(&self) -> f64 {
        self.sides_lens.iter().sum()
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

fn perimeter_by_area<T: Shape>(shape: T) -> f64 {
    shape.get_perimeter() / shape.get_area()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::relative_eq;

    #[test]
    fn test() {
        assert!(relative_eq!(
            perimeter_by_area(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            }),
            2.0,
            epsilon = 1e-4
        ));
        assert!(relative_eq!(
            perimeter_by_area(Circle { radius: 2.0 }),
            1.0,
            epsilon = 1e-4
        ));
        assert!(relative_eq!(
            perimeter_by_area(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
            1.6666,
            epsilon = 1e-4
        ));
    }
}
