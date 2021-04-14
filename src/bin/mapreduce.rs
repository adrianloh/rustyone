use std::iter::repeat;

#[derive(Debug)]
struct Thingee {
    order: u32,
}

fn main() {
    let a = vec!["( ❤U❤)(•́▿•̀ )".to_owned(); 1];
    let mut c = vec![];

    // `iter()` receives `&self` e.g. borrows an immutable reference
    a.iter(/* &self */)
        .for_each(|s: &String| {
            println!("{:p} -> {}", s, s);
        });

    // `into_iter()` receives `self` -- essentially moving `a`!
    a.into_iter(/* self */)
        .for_each(|s: String| {
            c.push(s);
        });
    // println!("{}", a) <- will not compile, because `a` was moved and dropped!

    let mut thingees: Vec<_> = (1..=10)
        .map(|i| Thingee { order: i })
        .inspect(|t| println!("{:?}", t))
        // Iterators are lazily evaluated. Functions like `collect()`,
        // `for_each()`, `count()` consume the iterator. They must be
        // at the end of the chain to "get the ball rolling"
        .collect();

    // Sort by struct field. If the sort closure is `a.prop.cmp(&b.prop)` the vector
    // is sorted low -> high. Swapping `a` for `b` reverses this order
    thingees.sort_by(|a, b| b.order.cmp(&a.order));

    thingees
        // `iter_mut()` when we want to change something in the iterable.
        // It receives a mutable reference to `self` 
        .iter_mut(/* &mut self */)
        .filter(|thingee| thingee.order > 5)
        .for_each(|thingee| thingee.order *= 100);

    // Take 5 times from an iterator that repeats its argument forever.
    // Collect the results -- a `Vec<String>` -- and `join()`
    println!("{}", repeat("🙃").take(5).collect::<Vec<_>>().join("😲"));

    let total = thingees
        .iter()
        // Note: `inspect()` always receives an immutable reference irregardless
        // of whether we started with `iter()` or `into_iter()`
        .inspect(|t| println!("{:?}", t))
        .count(); // Start the ball rolling, return the number of iterations

    println!("{}", vec!["🙄"; total].join(""));
}
