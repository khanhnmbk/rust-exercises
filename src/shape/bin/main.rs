use std::f64::consts::PI;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    // add fields
    x :i32,
    y :i32,
}

impl Point {
    pub fn new(x: i32, y : i32) -> Self {
        Self { x, y}
    }

    pub fn magnitude(&self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }

    pub fn dist(&self, other: &Self) -> f64 {
        let d = Self{x : self.x - other.x, y: self.y - other.y};
        d.magnitude()
    }

}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

pub struct Polygon {
    points : Vec<Point>,
}

impl Polygon {
    pub fn new() -> Self {
        Self {  points: Vec::new()}
    }

    pub fn add_point(&mut self, p : Point) {
        self.points.push(p);
    }

    pub fn left_most_point(&self) -> Option<Point>{
        /*
        let mut result : Option<Point> = None;

        for p in &self.points {
            if result.is_none() || p.x < result.unwrap().x  {
                result = Some(p.clone());
            }
        }
        return result;
        */

        self.points.iter().min_by_key(|p| p.x).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

    pub fn perimeter(&self) -> f64 {
        let mut p : f64 = 0.0;
        for (i, v) in self.points.iter().enumerate() {
            if i == self.points.len() - 1 {
                p += v.dist(&self.points[0]);
                break;
            }

            p += v.dist(&self.points[i+1]);
        }
        p
    }
}

pub struct Circle {
    central : Point,
    radius : i32,
}


impl Circle {
    pub fn new(central: Point, radius: i32) -> Self {
        Self {central, radius}
    }

    pub fn perimeter(&self) -> f64 {
        2.0*PI*(self.radius as f64)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(p) => p.perimeter(),
            Shape::Circle(c) => c.perimeter(),

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
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
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

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
