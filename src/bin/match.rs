#[derive(Debug)]
struct Color(i16, i16, i16);

struct Palette {
    selected: bool,
    color: Color,
}

enum A {
    Ready,
    Player(i64),
    One,
}

fn main() {
    // A few structures
    let vec_string = vec!["さび".to_owned(); 10];
    let vec_int = vec![1, 2, 3, 4];
    let vec_tuple = vec![("さび", 1), ("はい", 2), ("おげん", 3), ("なんて", 4)];
    let vec_structs = vec![
        Color(128, 54, 255),
        Color(43, 230, 244),
        Color(256, 34, 91),
        Color(0, 0, 0),
    ];
    let vec_enums = vec![A::Ready, A::Player(1), A::Player(2), A::One];

    // Literals
    match vec_string[0].as_str() {
        "さび" => println!("macthed literal"),
        _ => unreachable!(),
    };

    match vec_string[0].as_str() {
        // bind match to var `matched`
        matched @ "さび" => println!("matched @ {}", matched),
        _ => unreachable!(),
    };

    // Boolean
    vec_int.iter().for_each(|i| match i {
        // match 1, 2 or 3 -- bind to `m`
        m @ 1 | m @ 2 | m @ 3 => println!("bool: {}", m),
        x => println!("ignored: {}", x),
    });

    // Range
    match vec_int[0] {
        ref matched @ (0..=10) => println!("range: {}", matched),
        _ => panic!(),
    }

    // Guard expression
    match vec_string[0].as_str() {
        m if m == "さび" => println!("guard @ {}", m),
        _ => unreachable!(),
    };

    // Slice pattern
    match vec_int.as_slice() {
        [a, .., d] => println!("slice: {} {}", a, d),
        _ => unreachable!(),
    }

    // Tuple pattern
    vec_tuple.iter().for_each(|t| match t {
        (s, 1) => println!("1: {}", s),
        (s, 2) => println!("2: {}", s),
        (_, x) => println!("unknown: {}", x),
    });

    // Structs
    vec_structs.iter().for_each(|color| match color {
        Color(r, g, 200..=255) => println!("r: {}, g:{}", r, g),
        Color(0, 0, 0) => println!("black"),
        color => println!("reject: {:?}", color),
    });

    // Enums
    vec_enums.iter().for_each(|e| match e {
        A::Ready => println!("Ready"),
        A::Player(1) => println!("Player 1"),
        A::Player(2) => println!("Player 2"),
        _ => println!("ignore enum"),
    });

    // Nested decomposition

    let p1 = Palette {
        color: Color(30, 90, 43),
        selected: true,
    };

    let p2 = Palette {
        color: Color(30, 90, 43),
        selected: false,
    };

    let pp = vec![&p1, &p2];

    pp.iter().for_each(|p| match p {
        Palette {
            color: Color(r, g, b),
            selected: true,
        } => println!("ok: {} {} {}", r, g, b),
        Palette {
            color: c,
            selected: false,
        } => println!("reject: {:?}", c),
    });

    // Same as above - using `let` - since there's only one arm
    let Palette {
        color: Color(r, g, b),
        selected: ok,
    } = p1;
    println!("{}: {} {} {}", ok, r, g, b);
}
