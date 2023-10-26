use std::f64::consts::PI;

fn main(){

}
trait Shape1{
    fn rep_string(&self) -> String;
    fn area(&self) -> f64;
    fn clone_box(&self) -> Box<dyn Shape1>;
}

impl Clone for Box<dyn Shape1> {
    fn clone(&self) -> Box<dyn Shape1> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct Circle {
    x: f64,
    y: f64,
    r: f64
}

#[derive(Clone)]
struct Rectangle {
    x: f64,
    y: f64,
    w: f64,
    h: f64
}

#[derive(Clone)]
struct Triangle {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
}

impl Shape1 for Circle {
    fn rep_string(&self) -> String {
        format!("<Circle: {}, {}, {}>", self.x, self.y, self.r)
    }
    fn area(&self) -> f64 {
        PI * self.r.powi(2)
    }
    fn clone_box(&self) -> Box<dyn Shape1> {
        Box::new(self.clone())
    }
}

impl Circle {
    fn new(x: f64, y: f64, r: f64) ->  Box<dyn Shape1> {
        Box::new(Circle { x, y, r })
    }
}

impl Shape1 for Rectangle {
    fn rep_string(&self) -> String {
        format!("<Rectangle: {}, {}, {}, {}>", self.x, self.y, self.w, self.h)
    }
    fn area(&self) -> f64 {
        self.w * self.h
    }
    fn clone_box(&self) -> Box<dyn Shape1> {
        Box::new(self.clone())
    }
}

impl Rectangle {
    fn new(x: f64, y: f64, w: f64, h: f64) ->  Box<dyn Shape1> {
        Box::new(Rectangle { x, y, w, h })
    }
}

impl Shape1 for Triangle {
    fn rep_string(&self) -> String {
        format!("<Triangle: {}, {}, {}, {}, {}, {}>", self.x1, self.y1, self.x2, self.y2, self.x3, self.y3)
    }
    fn area(&self) -> f64 {
        0.5 * ((self.x1 - self.x3)*(self.y2 - self.y1) - (self.x1 - self.x2)*(self.y3 - self.y1))
    }
    fn clone_box(&self) -> Box<dyn Shape1> {
        Box::new(self.clone())
    }
}

impl Triangle {
    fn new(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) ->  Box<dyn Shape1> {
        Box::new(Triangle { x1, y1, x2, y2, x3, y3 })
    }
}

fn input_shape_list() -> Vec<Box<dyn Shape1>> {
    vec![
        Circle::new(0., 0., 1.), 
        Circle::new(50., 50., 15.),
        Rectangle::new(40., 40., 20., 20.), 
        Rectangle::new(10., 40., 15., 10.),
        Triangle::new(0., 0., 1., 0., 0., 1.)
    ]
}

const EXPECTED_001: &[&str] = &[
    "<Circle: 0, 0, 1>","<Circle: 50, 50, 15>",
    "<Rectangle: 40, 40, 20, 20>","<Rectangle: 10, 40, 15, 10>",
    "<Triangle: 0, 0, 1, 0, 0, 1>"
];
const EXPECTED_002: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
    "<Triangle: 0, 0, 1, 0, 0, 1>, area: 0.50",
];

#[test]
fn test_shapes_001() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(|s| s.rep_string());
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_001);
}

#[test]
fn test_shapes_002() {
    let shape_list = input_shape_list();
    let omap = shape_list.iter().map(
        |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}

#[test]
fn test_shapes_003() {
    let input_list = input_shape_list();
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(
        |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED_002);
}