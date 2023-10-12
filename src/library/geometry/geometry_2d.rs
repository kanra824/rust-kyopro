use num_complex;
use std::f64::consts::PI;

// 二次元幾何
type Point = num_complex::Complex64;
const EPS: f64 = 1e-8;

struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        Line { a, b }
    }

    // 点 p の直線 l への射影を求める
    fn projection(&self, p: &Point) -> Point {
        let a = dot(&(self.b - self.a), &(p - self.a));
        let b = dot(&(self.a - self.b), &(p - self.b));
        (a * self.b + b * self.a) / (a + b)
    }

    // 2直線の並行判定
    fn parallel(&self, l2: &Line) -> bool {
        eq(cross(&(self.a - self.b), &(l2.a - l2.b)), 0.0)
    }

    // 2直線の直行判定
    fn orthogonal(&self, l2: &Line) -> bool {
        eq(dot(&(self.a - self.b), &(l2.a - l2.b)), 0.0)
    }
}

struct Segment {
    a: Point,
    b: Point,
}

impl Segment {

}

impl Segment {
    fn new(a: Point, b: Point) -> Self {
        Segment { a, b }
    }
}

fn eq(a: f64, b: f64) -> bool {
    (b - a).abs() < EPS
}

fn radian_to_degree(r: f64) -> f64 {
    r * 180.0 / PI
}

fn degree_to_radian(d: f64) -> f64 {
    d * PI / 180.0
}

fn rotate(p: &Point, theta: f64) -> Point {
    p * Point::new(theta.cos(), theta.sin())
}

fn cross(a: &Point, b: &Point) -> f64 {
    a.re * b.im - a.im * b.re
}

fn dot(a: &Point, b: &Point) -> f64 {
    a.re * b.re + a.im * b.im
}




enum CCW {
    CounterClockwise,
    Clockwise,
    OnlineBack,
    OnlinFront,
    OnSegment,
}

// 有向線分と点の位置関係
// 線分 a->b からみて、点cがどこにあるか
fn ccw(a: &Point, b: &Point, c: &Point) -> CCW {
    let b = b - a;
    let c = c - a;
    if cross(&b, &c) > EPS {
        CCW::CounterClockwise
    } else if cross(&b, &c) < -EPS {
        CCW::Clockwise
    } else if dot(&b, &c) < 0.0 {
        CCW::OnlineBack
    } else if b.norm() < c.norm() {
        CCW::OnlinFront
    } else {
        CCW::OnSegment
    }
}

// 直線と点の交差判定
fn intersected(l: &Line, p: &Point) -> bool {
    match ccw(&l.a, &l.b, p) {
        CCW::CounterClockwise | CCW::Clockwise => false,
        _ => true,
    }
}

fn segment_intersected(s: &Segment, p: &Point) -> bool {
    match ccw(s, p) {
        CCW::OnSegment => true,
        _ => false,
    }
}