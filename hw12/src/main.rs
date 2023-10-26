use std::f64::consts::PI;

fn main() {
    let r = format!("[+]").repeat(3);
    println!("{}", r)
}

enum Shape {
    Circle(f64, f64, f64),
    Rectangle(f64, f64, f64, f64),
    Triangle(f64, f64, f64, f64, f64, f64)
}

impl Shape {
    fn rep_string(&self) -> String {
        match self {
            Shape::Circle(x,y ,r ) => format!("<Circle: {}, {}, {}>", x, y, r),
            Shape::Rectangle(x,y ,w ,h ) => format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h),
            Shape::Triangle(x1,y1 ,x2 ,y2 ,x3 ,y3 ) => format!("<Triangle: {}, {}, {}, {}, {}, {}>", x1, y1, x2, y2, x3, y3)
        }
    }
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(x,y ,r ) => PI * r.powi(2),
            Shape::Rectangle(x,y ,w ,h ) => w * h,
            Shape::Triangle(x1,y1 ,x2 ,y2 ,x3 ,y3 ) => 0.5 * ((x1 - x3)*(y2 - y1) - (x1 - x2)*(y3 - y1))
        }
    }
}

const INPUT_SHAPES: &[Shape] = &[
    Shape::Circle(0., 0., 1.),
    Shape::Circle(50., 50., 15.),
    Shape::Rectangle(40., 40., 20., 20.),
    Shape::Rectangle(10., 40., 15., 10.),
    // New Triangle instance
    Shape::Triangle(0., 0., 1., 0., 0., 1.),
];

const EXPECTED: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
    "<Triangle: 0, 0, 1, 0, 0, 1>, area: 0.50", // Expected result for Triangle
];

#[test]
fn test_shapes() {
    let input_list = INPUT_SHAPES;
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()));
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED);
}