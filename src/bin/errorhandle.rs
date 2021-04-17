use std::result::Result;

fn main() {
    // Brazenly unwrap and panic if we get an `Err`
    let mut x = k().unwrap();
    assert_eq!(x, 1);

    // Pattern match the `Result` enum
    x = match k() {
        Ok(i) => i + 1,
        Err(_) => unreachable!(),
    };
    assert_eq!(x, 2);

    // With a so-called `if let` statement
    x = if let Ok(i) = k() {
        i + 9
    } else {
        unreachable!()
    };
    assert_eq!(x, 10);

    // Vanilla `if...else` along with `is_err()` or `is_ok()`
    let result = k();
    if result.is_err() {
        unreachable!()
    }
    x = result.unwrap() + 19; // safe to unwrap
    assert_eq!(x, 20);

    // Unwrap, but return the zero-value of type in the case of an `Err`
    // The zero-value of `i64` is `0`
    x = e().unwrap_or_default();
    assert_eq!(x, 0);

    // Unwrap, but return a custom value in the case of en `Err`
    x = e().unwrap_or(1000);
    assert_eq!(x, 1000);

    // If the custom value is a function call e.g. `something.to_string()`
    // use `unwrap_or_else`. The closure is given the value wrapped in `Err`
    let z = es(&6969).unwrap_or_else(|err| *err);
    assert_eq!(z, 6969);

    // Look at definition of `f()` to see chaining and error propagation
    x = f(1).unwrap();
    assert_eq!(x, 40);

    // Panic on `Err` and print `message`
    let message = dying_message();
    x = e().expect(&message);
    unreachable!(x);
}

struct A {
    n: i64,
}

impl A {
    fn new(i: i64) -> Self {
        A { n: i }
    }
    fn add(mut self, i: i64) -> Result<Self, &'static str> {
        self.n += i;
        Ok(self)
    }
}

fn f(i: i64) -> Result<i64, &'static str> {
    // At each `?`, the unwrapped `Ok` is returned. If the chain fails at any point,
    // this function e.g. `f()` returns the `Err` -- propogating it to the caller
    let x = A::new(i).add(i)?.add(i)?.add(i)?.n;
    // The chain succeeded, we're free to do stuff with `x`
    let y = x * 10;
    Ok(y)
}

// Function always returns `Ok(1)`
fn k() -> Result<i64, &'static str> {
    Ok(1)
}

// Function always returns `Err()`
fn e() -> Result<i64, &'static str> {
    Err("------- Bazinga! -------")
}

// Function takes its argument and returns it wrapped in an `Err`
fn es(x: &'static i64) -> Result<i64, &'static i64> {
    Err(x)
}

// Just a fancy string
fn dying_message() -> String {
    let msg = "Hello darkness my old friend".to_string();
    let stars: String = vec!["*"; msg.len()].join("");
    format!("\n\n{}\n{}\n{}\n\n", stars, msg, stars)
}
