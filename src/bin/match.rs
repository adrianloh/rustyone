#[derive(Debug)]
struct Color(i16, i16, i16);

fn main() {
    // A few structures
    let vec_string = vec!["さび".to_owned(); 10];
    let vec_i = vec![1, 2, 3, 4];
    let vec_mix = vec![("さび", 1), ("さび", 2), ("さび", 3), ("さび", 4)];
    let vec_structs = vec![
        Color(128, 54, 255),
        Color(43, 230, 244),
        Color(256, 34, 91),
        Color(0, 0, 0),
    ];

    // Literals
    match vec_string[0].as_str() {
        "さび" => println!("literal: さび"),
        _ => unreachable!(),
    };

    match vec_string[0].as_str() {
        // binding to a variable `matched`
        matched @ "さび" => println!("matched @ {}", matched),
        _ => unreachable!(),
    };

    // Boolean
    vec_i.iter().for_each(|i| match i {
        // match 1 or 2 -- bind to `m`
        m @ 1 | m @ 2 => println!("bool: {}", m),
        x => println!("ignored: {}", x),
    });

    // Range
    match vec_i[0] {
        ref matched @ (0..=10) => println!("range: {}", matched),
        _ => panic!(),
    }

    // Guard
    match vec_string[0].as_str() {
        m if m == "さび" => println!("guard @ {}", m),
        _ => unreachable!(),
    };

    // Slice pattern
    match vec_i.as_slice() {
        [a, .., d] => println!("slice: {} {}", a, d),
        _ => unreachable!(),
    }

    // Tuple pattern
    vec_mix.iter().for_each(|t| match t {
        (s, 1) => println!("1: {}", s),
        (s, 2) => println!("2: {}", s),
        (_, x) => println!("unknown: {}", x),
    });

    // Structs
    vec_structs.iter().for_each(|color| match color {
        Color(r, g, 200..=255) => println!("r: {}, g:{}", r, g),
        Color(0, 0, 0) => println!("got_black"),
        color => println!("unknown: {:?}", color),
    });
}
