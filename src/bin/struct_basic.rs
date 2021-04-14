// `Copy` allows the struct to be passed by value or reassigned
// `Clone` allows `.clone()`
// `PartialEq` allows us to `==` or `!=` between `Point`s since all fields of `Point` are comparable

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        // Constructor
        Point { x, y }
    }
    fn magnitude(&self) -> f64 {
        // Using `&` since we just need a read-only ref to self
        let xsq = self.x * self.x;
        let ysq = self.y * self.y;
        (xsq + ysq).sqrt()
    }
    fn translate(&mut self, x: f64, y: f64) {
        // Using `&mut` since we're mutating self
        self.x += x;
        self.y += y;
    }
}

fn main() {
    let mut p1 = Point::new(10.0, 10.0);
    println!("{:?}, magnitude: {}", p1, p1.magnitude());

    p1.translate(10.0, 10.0);
    println!("translated: {:?}", p1);

    let p2 = Point { x: 20.0, y: 20.0 };
    assert_eq!(p1 == p2, true)
}
