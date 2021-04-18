use rand::{distributions::Alphanumeric, Rng};

// `A` acts as a container for a static function `f` that returns
// different typed values based on the type we assign it to
trait A {
    fn f(n: usize) -> Self;
}

impl A for Vec<char> {
    fn f(n: usize) -> Vec<char> {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(n)
            .map(char::from)
            .collect()
    }
}

impl A for Vec<i64> {
    fn f(n: usize) -> Vec<i64> {
        (0..).take(n).collect()
    }
}

fn main() {
    let a: Vec<i64> = A::f(10);
    let b: Vec<char> = A::f(10);

    println!("{:>10} : {:?}", "Vec<i64>", a);
    println!("{:>10} : {:?}", "Vec<char>", b);
}
