use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Delta {
    dx: f64,
    dy: f64,
}

impl Add<Delta> for Point {
    type Output = Point;

    fn add(self, delta: Delta) -> Point {
        Point {
            x: self.x + delta.dx,
            y: self.y + delta.dy,
        }
    }
}

impl Sub<Delta> for Point {
    type Output = Point;

    fn sub(self, delta: Delta) -> Point {
        Point {
            x: self.x - delta.dx,
            y: self.y - delta.dy,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Delta;

    fn sub(self, other: Point) -> Delta {
        Delta {
            dx: self.x - other.x,
            dy: self.y - other.y,
        }
    }
}

impl Mul<f64> for Delta {
    type Output = Delta;

    fn mul(self, scalar: f64) -> Delta {
        Delta {
            dx: self.dx * scalar,
            dy: self.dy * scalar,
        }
    }
}

impl Mul<Delta> for f64 {
    type Output = Delta;

    fn mul(self, delta: Delta) -> Delta {
        Delta {
            dx: delta.dx * self,
            dy: delta.dy * self,
        }
    }
}

impl Div<f64> for Delta {
    type Output = Delta;

    fn div(self, scalar: f64) -> Delta {
        Delta {
            dx: self.dx / scalar,
            dy: self.dy / scalar,
        }
    }
}
