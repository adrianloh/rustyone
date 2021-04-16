use std::iter::repeat;

#[derive(Debug)]
struct Thingee {
    order: u32,
}

fn main() {
    // ## `iter()` , `into_iter()` and references

    let stringee = "( ❤U❤)(•́▿•̀ )".to_owned();

    // `a` is vector of Strings
    let a: Vec<String> = vec![stringee.clone()];

    // `iter()` receives `&self` e.g. it borrows an immutable reference to `a`
    a.iter(/* &self */)
        .for_each(|s: &String| {
            borrow_string(s)
        });

    // `into_iter()` receives `self` -- essentially moves `a`!
    a.into_iter(/* self */)
        .for_each(|s: String| {
            move_string(s);
        });
    // println!("{}", a) [!!!] will not compile, because `a` was moved into `into_iter()` and dropped!

    // `b` is a vector of references to Strings
    let b: Vec<&String> = vec![&stringee];
    let mut refs: Vec<&String> = vec![];

    b.iter().for_each(|ss: &&String| {
        // `ss` is a reference to a reference 🧐 since we used `iter()`.
        // Dereferencing into `s` gives us the original reference
        let s /*&String*/ = *ss;
        println!("{:p} -> {:p}", ss, s);
        refs.push(s);
    });

    b.into_iter().for_each(|s: &String| {
        // `s` is already a reference, no need to dereference
        println!("{:p}", s);
        refs.push(s);
    });
    // println!("{}", b) [!!!] will not compile, because `b` was moved into `into_iter()` and dropped!

    assert_eq!(refs[0], refs[1]);

    // `c` is vector of Strings
    let c: Vec<String> = vec![stringee];

    // a `for` loop with `&` is like `iter(&self)`
    for s in &c {
        // `s` is `&String`
        borrow_string(s)
    }

    // a `for` loop _without_ `&` is like `into_iter(self)`
    for s in c {
        // `s` is `String`
        move_string(s)
    }
    // Once again, `c` has moved and is bye bye, so is `stringee` since we moved it into `c` 😭!

    // ## Iterator functions/adapters

    // Build a vector of Thingees with map/collect
    let mut thingees: Vec<Thingee> = (1..=10)
        .map(|i| Thingee { order: i })
        .inspect(|t| println!("{:?}", t))
        // Iterators are lazily evaluated. Functions like `collect()`,
        // `for_each()`, `count()` consume the iterator. They're always
        // at the end of the chain to "get the ball rolling"
        .collect();

    // Sort by struct field. If the sort closure is `a.prop.cmp(&b.prop)` the vector
    // is sorted low -> high. Swapping `a` for `b` reverses this order
    thingees.sort_by(|a, b| b.order.cmp(&a.order));

    thingees
        // Use `iter_mut()` when you want to change something in the iterable.
        // `iter_mut()` receives a mutable reference to `self`
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

    // Reduce/fold
    let orders = thingees
        .iter()
        // push each `thingee.order` into the "orders" vector
        .fold(Vec::new(), |mut orders, t| {
            orders.push(t.order);
            orders
        });

    // Inspect orders
    println!("orders: {:?}", orders);

    // Sum the vector
    println!("total order: {}", orders.iter().sum::<u32>());
}

fn borrow_string(s: &str) {
    println!("borrow: {}", s);
}

fn move_string(s: String) {
    println!("moved: {}", s);
}
