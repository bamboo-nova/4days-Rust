#![allow(unused_variables, dead_code)]
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point {
            x: x as f64,
            y: y as f64,
        }
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(
            f64::powf(self.x, 2.0) + f64::powf(self.y, 2.0)
        )
    }

    pub fn dist(&self, other: Point) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        f64::sqrt(
            f64::powf(dx, 2.0) + f64::powf(dy, 2.0)
        )
    }
}

trait Circumference {
    fn circumference(&self) -> f64;
}

pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    pub fn new() -> Self {
        Self {
            points: vec![],
        }
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn left_most_point(&self) -> Option<Point> {
        // cloned()にしてOptionにする
        self.points.iter().min_by(|a, b| a.x.partial_cmp(&b.x).unwrap()).cloned()
    }
}

impl Circumference for Polygon {
    fn circumference(&self) -> f64 {
        let p1 = &self.points;
        let mut p2 = self.points.clone();

        p2.rotate_right(1);

        let mut sum = 0.0;

        for i in 0..p1.len() {
            sum += p1[i].dist(p2[i]);
        }

        sum
    }
}

pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: i64) -> Self {
        Self {
            center,
            radius: radius as f64,
        }
    }
}

impl Circumference for Circle {
    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(polygon: Polygon) -> Self {
        Shape::Polygon(polygon)
    } 
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    } 
}

impl Circumference for Shape {
    fn circumference(&self) -> f64 {
        match self {
            Shape::Polygon(polygon) => polygon.circumference(),
            Shape::Circle(circle) => circle.circumference(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);
        let p3 = Point::new(17, 17);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        poly.add_point(p3);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.points.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let circumferences = shapes
            .iter()
            .map(Shape::circumference)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(circumferences, vec![15.48, 31.42]);
    }

}

#[allow(dead_code)]
fn main() {}
