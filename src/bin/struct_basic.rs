// `Copy` allows the struct to be passed by value or reassigned
// `Clone` allows `.clone()` and _must_ be implemented to allow `Copy`
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
    #[allow(dead_code)]
    fn moveout(self) -> Self {
        // return `self` -- essentially moving out!
        self
    }
}

fn main() {
    let mut p1 = Point::new(10.0, 10.0);
    println!("{:?}, magnitude: {}", p1, p1.magnitude());

    p1.translate(10.0, 10.0);
    println!("translated: {:?}", p1);

    let p2 = Point { x: 20.0, y: 20.0 };
    assert_eq!(p1 == p2, true);

    // Without `Copy`, this would move `p1` and `p2` into `points`...
    let points = vec![p1, p2];
    // `p1` and `p2` are still around!
    assert_eq!(p1 == p2, true);

    let mut points_copies: Vec<Point> = Vec::new();
    let mut points_clones: Vec<Point> = Vec::new();
    points.iter().for_each(|point /* &Point */| {
        // `Copy` allows us to...
        let pp1 = *point; // dereference _and_ move `*point` into `pp1`, again copying it
        let pp2 = pp1; // `pp1` doesn't move, it's copied into `pp2`
        points_copies.push(pp1);
        points_clones.push(pp2);
    });
    assert_eq!(points, points_copies);
    assert_eq!(points_copies, points_clones);
}
