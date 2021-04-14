use std::iter::repeat;

#[derive(Debug)]
struct Thingee {
    order: u32,
}

fn main() {
    let mut thingees: Vec<_> = (1..=10)
        .map(|i| Thingee { order: i })
        .inspect(|t| println!("{:?}", t))
        .collect();

    thingees.sort_by(|a, b| b.order.cmp(&a.order));

    thingees
        .iter_mut()
        .filter(|thingee| thingee.order > 5)
        .for_each(|thingee| thingee.order *= 100);

    println!("{}", repeat("🙃").take(5).collect::<Vec<_>>().join("😲"));

    let total = thingees.iter().inspect(|t| println!("{:?}", t)).count();

    println!("{}", vec!["🙄"; total].join(""));
}
