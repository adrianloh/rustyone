struct A(i64);
struct B(String);

trait Messy {
    // Must be defined by implementors
    fn first(&self) -> i64;
    // You get this for free
    fn print_first(&self) {
        println!("My first member is: {:?}", self.first())
    }
    // Also FOC
    fn first_plus_(&self, i: i64) -> i64 {
        self.first() + i
    }
}

impl Messy for A {
    fn first(&self) -> i64 {
        self.0
    }
}

impl Messy for B {
    fn first(&self) -> i64 {
        self.0.parse::<i64>().unwrap_or(0)
    }
}

fn plus(x: &impl Messy, i: i64) -> i64 {
    x.first_plus_(i)
}

fn main() {
    let a = A(32);
    a.print_first();

    let b = B("1979".to_owned());
    b.print_first();

    // same as `plus` except we can't use `impl Messy` in a closure's signature
    let call_print = |t: &dyn Messy| t.print_first();

    // A vector of Messy-ies -- in this case, Rust needs to be hinted
    let messies: Vec<&dyn Messy> = vec![&a, &b];

    messies.iter().for_each(|o| call_print(*o));

    let x = plus(&b, 21);
    assert_eq!(x, 2000);
}
