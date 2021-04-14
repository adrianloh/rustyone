use anyhow::anyhow;
use anyhow::Result;

fn main() {
    // Brazenly unwrap and panic if we get an `Err`
    let mut x = a().unwrap();
    assert_eq!(x, 1);

    // With pattern matching
    x = match a() {
        Ok(i) => i + 1,
        Err(_) => panic!(),
    };
    assert_eq!(x, 2);

    // With a so-called `if let` statement
    x = if let Ok(i) = a() { i + 9 } else { panic!() };
    assert_eq!(x, 10);

    // Vanilla `if...else` along with `is_err()` or `is_ok()`
    let result = a();
    if result.is_err() {
        panic!()
    }
    x = result.unwrap() + 19; // safe to unwrap
    assert_eq!(x, 20);

    // Unwrap, but return a default value in the case of an `Err`
    x = e().unwrap_or(0);
    assert_eq!(x, 0);

    // Like `unwrap_or`, but the return value is computed from the closure
    // The closure is given the value wrapped in `Err`
    x = f(1).unwrap_or_else(|_err| 0);
    assert_eq!(x, 40);

    // Panic on `Err` and print `dying_message()`
    x = e().expect(&dying_message());
    assert_eq!(x, 40); // We never get here
}

struct A {
    n: i64,
}

impl A {
    fn new(i: i64) -> Self {
        A { n: i }
    }
    fn add(mut self, i: i64) -> Result<Self> {
        self.n += i;
        Ok(self)
    }
}

fn f(i: i64) -> Result<i64> {
    // At each `?`, the unwrapped `Ok` is returned. If the chain fails at any point,
    // this function e.g. `f` returns the `Err`, propogating it to the caller
    let x = A::new(i).add(i)?.add(i)?.add(i)?.n;
    // The chain succeeded, we're free to do stuff with `x`
    let y = x * 10;
    Ok(y)
}

fn a() -> Result<i64> {
    Ok(1)
}

fn e() -> Result<i64> {
    Err(anyhow!("------- Bazinga! -------"))
}

fn dying_message() -> String {
    let msg = "Hello darkness my old friend";
    let stars = (0..msg.len()).map(|_| "*").collect::<Vec<&str>>().join("");
    format!("\n\n{}\n{}\n{}\n\n", stars, msg, stars)
}
