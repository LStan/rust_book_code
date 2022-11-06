use std::ops::Add;

// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, other: (i32, i32)) -> Self {
        Point {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

impl Add<Point> for (i32, i32) {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.0 + other.x,
            y: self.1 + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(
        Point { x: 1, y: 0 } + (2, 3),
        Point { x: 3, y: 3 }
    );

    assert_eq!(
        (2, 3) + Point { x: 1, y: 0 },
        Point { x: 3, y: 3 }
    );
}
