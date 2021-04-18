use std::fmt::Debug;

fn doless<T: Debug>(x: T) -> impl Debug {
    format!("{:?}", x)
}

fn main() {
    let s: String = String::from("fuck");
    let f /* &str */ = &*s;
    let t = "shit";
    let j = format!("{}{}", s, t);
    let ww = vec![f; 10].join(",");
    println!("{:?}", j);
    println!("{:?}", ww);
    let tot = (1..=10).map(|i| println!("{}", i)).count();
    println!("# {}", tot);
}
