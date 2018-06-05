use std::ops::{Add, Div, Mul, Sub};

/// Representation of a location in the source image, in pixels
#[derive(Debug, Copy, Clone)]
pub struct Point {
    /// X Coordinate, in pixels
    pub x: f64,

    /// Y Coordinate, in pixels
    pub y: f64,
}

/// Difference between two [`Point`]s, in pixels
#[derive(Debug, Copy, Clone)]
pub struct Delta {
    /// X Coordinate difference, in pixels    
    pub dx: f64,

    /// Y Coordinate difference, in pixels
    pub dy: f64,
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

impl Add<Delta> for Delta {
    type Output = Delta;

    fn add(self, delta: Delta) -> Delta {
        Delta {
            dx: self.dx + delta.dx,
            dy: self.dy + delta.dy,
        }
    }
}

impl Sub<Delta> for Delta {
    type Output = Delta;

    fn sub(self, delta: Delta) -> Delta {
        Delta {
            dx: self.dx - delta.dx,
            dy: self.dy - delta.dy,
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
