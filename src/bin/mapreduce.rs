use std::iter::repeat;

#[derive(Debug)]
struct Thingee {
    order: u32,
}

fn move_string(s: String) {
    println!("{}", s);
}

fn ref_string(s: &str) {
    println!("{}", s);
}

fn main() {
    /* `iter()` , `into_iter()` and references */

    // `a` is vector of Strings
    let a: Vec<String> = vec!["( ❤U❤)(•́▿•̀ )".to_owned()];

    // `iter()` receives `&self` e.g. borrows an immutable reference
    a.iter(/* &self */)
        .for_each(|s: &String| {
            ref_string(s)
        });

    // `into_iter()` receives `self` -- essentially moving `a`!
    a.into_iter(/* self */)
        .for_each(|s: String| {
            move_string(s);
        });
    // println!("{}", a) <- will not compile, because `a` was moved into `into_iter()` and dropped!

    // `b` is vector of references to Strings
    let stringee = "( ❤U❤)(•́▿•̀ )".to_owned();
    let b: Vec<&String> = vec![&stringee];
    let mut refs = vec![];

    b.iter().for_each(|ss: &&String| {
        // `ss` is a reference to a reference since we used `iter()`.
        // Dereferencing into `s` gives us the original reference
        let s /*&Thingee*/ = *ss;
        println!("{:p} -> {:p}", ss, s);
        refs.push(s);
    });

    b.into_iter().for_each(|s: &String| {
        // `s` is already a reference, no need to dereference
        println!("{:p}", s);
        refs.push(s);
    });
    // println!("{}", b) <- will not compile, because `b` was moved into `into_iter()` and dropped!

    assert_eq!(refs[0], refs[1]);

    /* iterator functions/adapters */

    // Build a vector of Thingees with map/collect
    let mut thingees: Vec<Thingee> = (1..=10)
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
    // Collect the results and `join()`
    println!("{}", repeat("🙃").take(5).collect::<Vec<&str>>().join("😲"));

    let total = thingees
        .iter()
        // Note: `inspect()` always receives an immutable reference irregardless
        // of whether we started with `iter()` or `into_iter()`
        .inspect(|t| println!("{:?}", t))
        .count(); // Start the ball rolling, return the number of iterations

    println!("{}", vec!["🙄"; total].join(""));
}
