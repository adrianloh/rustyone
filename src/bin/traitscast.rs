trait A {
    fn f(n: usize) -> Self;
}

impl A for Vec<&str> {
    fn f(n: usize) -> Vec<&'static str> {
        vec!["whaa"; n]
    }
}

impl A for Vec<i64> {
    fn f(n: usize) -> Vec<i64> {
        (0..).take(n).collect()
    }
}

fn main() {
    // The return value of `A::f()` depends on the
    // type we assign it to
    let a: Vec<i64> = A::f(10);
    println!("Vec<i64>: {:?}", a);
    let b: Vec<&str> = A::f(10);
    println!("Vec<&str>: {:?}", b);
}
