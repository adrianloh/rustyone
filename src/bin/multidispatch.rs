#[derive(Debug)]
struct Point(f64, f64, f64);
struct Scale(f64);
struct Translate(f64, f64, f64);

trait Apply<T> {
    fn apply(&mut self, _: T);
}

impl Apply<Scale> for Point {
    fn apply(&mut self, scale: Scale) {
        self.0 *= scale.0;
        self.1 *= scale.0;
        self.2 *= scale.0;
    }
}

impl Apply<Translate> for Point {
    fn apply(&mut self, translate: Translate) {
        self.0 += translate.0;
        self.1 += translate.1;
        self.2 += translate.2;
    }
}

fn main() {
    let mut p = Point(0.0, 0.0, 0.0);
    p.apply(Translate(1.0, 1.0, 1.0));
    println!("{:?}", p);
    p.apply(Scale(10.0));
    println!("{:?}", p);
}
