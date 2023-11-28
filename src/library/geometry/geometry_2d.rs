use num_complex::{self, Complex64};
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

    // 直線と点の交差判定
    fn point_intersected(&self, p: &Point) -> bool {
        match ccw(&self.a, &self.b, p) {
            CCW::CounterClockwise | CCW::Clockwise => false,
            _ => true,
        }
    }

    // 直線と線分の交差判定
    fn segment_intersected(&self, s: &Segment) -> bool {
        let c1 = cross(&(self.b - self.a), &(s.a - self.a));
        let c2 = cross(&(self.b - self.a), &(s.b - self.a));
        c1 * c2 < EPS
    }

    // 2直線の交点
    fn crosspoint(&self, l: &Line) -> Point {
        let c1 = cross(&(l.a - self.a), &(l.b - self.a));
        let c2 = cross(&(l.b - self.b), &(l.a - self.b));
        (c1 * self.b + c2 * self.a) / (c1 + c2)
    }

    // 直線と点の距離
    fn point_distance(&self, p: &Point) -> f64 {
        (self.projection(p) - p).norm()
    }

    // 直線と線分の距離
    fn segment_distance(&self, s: &Segment) -> f64 {
        if self.segment_intersected(s) {
            0.0
        } else {
            let d1 = self.point_distance(&s.a);
            let d2 = self.point_distance(&s.b);
            d1.min(d2)
        }
    }
}

struct Segment {
    a: Point,
    b: Point,
}

impl Segment {
    fn new(a: Point, b: Point) -> Self {
        Segment { a, b }
    }

    // 点 p の直線 l への射影を求める
    fn projection(&self, p: &Point) -> Point {
        segment_to_line(self).projection(p)
    }

    // 2直線の並行判定
    fn parallel(&self, l2: &Line) -> bool {
        segment_to_line(self).parallel(l2)
    }

    // 2直線の直行判定
    fn orthogonal(&self, l2: &Line) -> bool {
        segment_to_line(self).orthogonal(l2)
    }

    // 線分と点の交差判定
    fn point_intersected(&self, p: &Point) -> bool {
        match ccw(&self.a, &self.b, p) {
            CCW::OnSegment => true,
            _ => false,
        }
    }

    // 2つの線分の交差判定
    fn segment_intersected(&self, s: &Segment) -> bool {
        let ccw1 = ccw(&self.a, &self.b, &s.a);
        let ccw2 = ccw(&self.a, &self.b, &s.b);
        let ccw3 = ccw(&s.a, &s.b, &self.a);
        let ccw4 = ccw(&s.a, &s.b, &self.b);
        (ccw1 as i32) * (ccw2 as i32) <= 0 && (ccw3 as i32) * (ccw4 as i32) <= 0
    }

    // 線分と点の距離
    fn point_distance(&self, p: &Point) -> f64 {
        let r = self.projection(p);
        if self.point_intersected(&r) {
            (r - p).norm()
        } else {
            let d1 = (self.a - p).norm();
            let d2 = (self.b - p).norm();
            d1.min(d2)
        }
    }

    // 2つの線分の距離
    fn segment_distance(&self, s: &Segment) -> f64 {
        if self.segment_intersected(s) {
            0.0
        } else {
            let d1 = self.point_distance(&s.a);
            let d2 = self.point_distance(&s.b);
            let d3 = s.point_distance(&self.a);
            let d4 = s.point_distance(&self.b);
            d1.min(d2).min(d3).min(d4)
        }
    }
}

struct Circle {
    p: Point,
    r: f64,
}

impl Circle {
    fn new(p: Point, r: f64) -> Self {
        Circle { p, r }
    }

    // 2つの円の交差の状態
    fn intersected(&self, c: &Circle) -> usize {
        let (c1, c2) = if self.r < c.r { (self, c) } else { (c, self) };

        let d = (c1.p - c2.p).norm();
        if c1.r + c2.r < d {
            // 交差なし
            4
        } else if eq(c1.r + c2.r, d) {
            // 外接
            3
        } else if c1.r - c2.r < d {
            // 2点交差
            2
        } else if eq(c1.r - c2.r, d) {
            // 内接
            1
        } else {
            // 内包
            0
        }
    }

    // 円と直線の交点
    fn line_crosspoint(&self, l: &Line) -> Vec<Point> {
        let h = l.point_distance(&self.p);
        let p = l.projection(&self.p);
        if eq(h, self.r) {
            return vec![p];
        }

        let mut u = l.a - l.b;
        u /= u.norm();
        let d = (self.r * self.r - h * h).sqrt();
        vec![p + d * u, p - d * u]
    }

    // 2つの円の交点
    fn circle_crosspoint(&self, c: &Circle) -> Vec<Point> {
        let intersection = self.intersected(c);
        if intersection == 4 || intersection == 0 {
            return vec![];
        }
        let d = (self.p - c.p).norm();
        let t = (c.p - self.p).arg();
        let a = (self.r * self.r - c.r * c.r + d * d) / (2.0 * self.r * d);

        if intersection == 2 {
            vec![
                self.p + Complex64::from_polar(self.r, t + a),
                self.p + Complex64::from_polar(self.r, t - a),
            ]
        } else {
            vec![self.p + Complex64::from_polar(self.r, t)]
        }
    }

    // 点p, 円c に対し、pを通るcの接線を返す(c上の2点)
    // 点pが円cの外側にあることを確認してから呼ぶこと
    fn tangent(&self, p: Point) -> Vec<Point> {
        unimplemented!();
    }

    // 2つの円に共通する接線を返す(最大4本)
    fn common_tangent(&self, c: &Circle) -> Vec<Line> {
        unimplemented!();
    }
}

fn segment_to_line(s: &Segment) -> Line {
    Line::new(s.a, s.b)
}

fn line_to_segment(l: &Line) -> Segment {
    Segment::new(l.a, l.b)
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

impl From<CCW> for i32 {
    fn from(ccw: CCW) -> Self {
        match ccw {
            CCW::CounterClockwise => 1,
            CCW::Clockwise => -1,
            CCW::OnlineBack => 2,
            CCW::OnlinFront => -2,
            CCW::OnSegment => 0,
        }
    }
}

impl CCW {
    fn to_i32(&self) -> i32 {
        match self {
            CCW::CounterClockwise => 1,
            CCW::Clockwise => -1,
            CCW::OnlineBack => 2,
            CCW::OnlinFront => -2,
            CCW::OnSegment => 0,
        }
    }
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
