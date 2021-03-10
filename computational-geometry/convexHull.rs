use std::{clone::Clone, cmp::PartialEq, ops::Sub};

const N: usize = 10;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Sub for Point {
    type Output = Point;
    
    fn sub(self, other:Point) -> Point{
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }  
}

fn min_x(s: &[Point]) -> Point {
    let mut min = &s[0];
    for i in 1..N {
        if s[i].x < (*min).x {
            min = &s[i];
        }
    }
    return *min;
}

fn cross (u:Point, v:Point) -> i32{
    return u.x*v.y - u.x*v.y;
}

fn main() {
    let s: [Point; N] = [
        Point::new(15, 30),
        Point::new(13, 42),
        Point::new(6, 27),
        Point::new(8, 5),
        Point::new(22, 37),
        Point::new(20, 40),
        Point::new(2, 5),
        Point::new(40, 6),
        Point::new(34, 20),
        Point::new(8, 13),
    ]; 
    jarvis(s);
} 

fn jarvis(s: [Point;N]) {
    //min value is copied to point_on_hull
    let mut point_on_hull = min_x(&s);
    let mut p: Vec<Point> = Vec::new();
    let mut i = 0;
    loop {
        p.push(point_on_hull);
        let mut endpoint = s[0];
        for j in 0..N {
            if endpoint == point_on_hull || cross(endpoint - p[i], s[j] - p[i]) > 0 {
                endpoint = s[j];
            }
        }
        i += 1;
        point_on_hull = endpoint;
        if endpoint == p[0] {
            break;
        }
    }
    
    println!("{:?}",  p);
}
