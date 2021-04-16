#[derive(Debug)]
struct A(i64);

#[derive(Debug)]
struct B(String);

trait Messy {
    // Must be defined by implementors
    fn first(&self) -> i64;
    // By implementing `first()`, you get this for free
    fn print_first(&self) {
        println!("My first member is: {:?}", self.first())
    }
    // Also free!
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

// Just an empty wrapper for Messify's methods
//  What's the better way?
struct C();

trait Messify<T> {
    // We cannot simply return `-> dyn Messy` because whatever that's
    // Messy (which could be a gazillion different types) cannot be
    // determined at compile time (E0277). So, we need to `Box` the
    // `Messy` thing up.
    fn messify(_: T) -> Box<dyn Messy>;
}

impl Messify<&str> for C {
    fn messify(s: &str) -> Box<dyn Messy> {
        Box::new(B(s.to_string()))
    }
}

impl Messify<i64> for C {
    fn messify(i: i64) -> Box<dyn Messy> {
        Box::new(A(i))
    }
}

// A "generic function" that calls Messy's `first_plus_()` with `i`
fn plus(x: &impl Messy, i: i64) -> i64 {
    x.first_plus_(i)
}

// An alternative way of defining `plus` using `fn<T>`
fn plusplus<T: Messy>(x: &T, i: i64) -> i64 {
    plus(x, i)
}

// Yet another way of defining a function like `plus` using `where`
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

    // A vector of references to Messy-ables -- in this case,
    // Rust needs to be type hinted
    let messies: Vec<&dyn Messy> = vec![&a, &b];
    messies.iter().for_each(|o /*&&dyn Messy*/| call_print(*o));

    let mut x = plus(&b, 21);
    assert_eq!(x, 2000);

    x = plusplus(&b, 21);
    assert_eq!(x, 2000);

    x = minus(&a, 2020);
    assert_eq!(x, 0);

    // `C::messify()` returns "something" that implements `Messy` (depending on what we
    // pass in). We cannot know what exactly is Messy¹ e.g. (is it an `A` or `B`?)
    // but we can call all the methods associated with being `Messy`.
    let m1 = C::messify(43);
    assert_eq!(m1.first(), 43);

    let m2 = C::messify("1979");
    assert_eq!(m2.first(), 1979);

    // ¹ It *is* possible to get the underlying type with `c_a.downcast_ref::<A>()`
    // but that's a whole different bag of worms.
}
