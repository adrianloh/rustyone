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

// A "generic function" that calls Messy's `first_plus_()` with `i`
fn plus(x: &impl Messy, i: i64) -> i64 {
    x.first_plus_(i)
}

// An alternative way of defining a function like `plus` using `where`.
// Takes a Messy's first member and subtracts `i` from it
fn minus<T>(x: &T, i: i64) -> i64
where
    T: Messy,
{
    x.first() - i
}

fn main() {
    let a = A(2020);
    let b = B("1979".to_owned());

    // a "generic closure" -- same as `plus` except we can't
    // use `impl Messy` in a closure's signature, so we use `dyn Messy`
    let call_print = |t: &dyn Messy| t.print_first();

    // A vector of references to Messy-ables -- in this case, Rust needs to be hinted
    let messies: Vec<&dyn Messy> = vec![&a, &b];

    messies.iter().for_each(|o /*&&dyn Messy*/| call_print(*o));

    let mut x = plus(&b, 21);
    assert_eq!(x, 2000);

    x = minus(&a, 2020);
    assert_eq!(x, 0);
}
