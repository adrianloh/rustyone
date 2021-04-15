#![allow(dead_code)]

const BETINA: &str = "https://gist.githubusercontent.com/adrianloh/d85483b8d561397d03adc89f30943dcc/raw/010913dd40b3c8556b149b91b0a5a486d6764cde/females.txt";

struct A(i64);
struct B(String);

trait Thingable {
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

impl Thingable for A {
    fn first(&self) -> i64 {
        self.0
    }
}

impl Thingable for B {
    fn first(&self) -> i64 {
        self.0.parse::<i64>().unwrap_or(0)
    }
}

fn plus(x: &impl Thingable, i: i64) -> i64 {
    x.first_plus_(i)
}

fn main() {
    let a = A(32);
    a.print_first();

    let b = B("1979".to_owned());
    b.print_first();

    // exactly the same as `plus` except we can't use `impl Thingaable` in a closure's signature
    let call_print = |t: &dyn Thingable| t.print_first();

    call_print(&a);

    let x = plus(&b, 21);
    assert_eq!(x, 2000);
}

fn get_names() -> Result<Vec<String>, ureq::Error> {
    let body = ureq::get(BETINA).call()?.into_string()?;
    Ok(body
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>())
}
