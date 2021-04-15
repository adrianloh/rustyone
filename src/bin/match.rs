#[derive(Debug)]
struct Color(i16, i16, i16);

#[derive(Debug)]
struct Palette {
    selected: bool,
    color: Color,
}

#[derive(Debug)]
enum A {
    Ready,
    Player(i64),
    One,
}

fn main() {
    // A few structures to start
    let vec_string = vec!["さび".to_owned(); 10];
    let vec_int = vec![1, 2, 3, 4];
    let vec_tuple = vec![("さび", 1), ("はい", 2), ("おげん", 3), ("なんて", 4)];
    let vec_enums = vec![A::Ready, A::Player(1), A::Player(2), A::One];
    let vec_structs = vec![
        Color(128, 54, 255),
        Color(43, 230, 244),
        Color(256, 34, 91),
        Color(0, 0, 0),
    ];

    // Literals
    match vec_string[0].as_str() {
        "さび" => println!("matched literal"),
        _ => unreachable!(),
    };

    match vec_string[0].as_str() {
        // bind match to var `matched`
        matched @ "さび" => println!("matched {}", matched),
        _ => unreachable!(),
    };

    // Boolean
    vec_int.iter().for_each(|i| match i {
        // match 1, 2 or 3 -- bind to `m` -- then match `m`
        m @ 1 | m @ 2 | m @ 3 => match m {
            1 => println!("We got one!"),
            2 => println!("We got two!"),
            3 => println!("We got three!"),
            _ => unreachable!(),
        },
        x => println!("ignored: {}", x),
    });

    // Range
    match vec_int[0] {
        ref m @ 0..=10 => println!("range 1-10: {}", m),
        ref m @ 11..=20 => unreachable!(m),
        _ => unreachable!(),
    }

    // Guard expression
    match vec_string[0].as_str() {
        m if m == "さび" => println!("guard expression: {}", m),
        _ => unreachable!(),
    };

    // Guard as an extra conditional
    match vec_string[0].as_str() {
        m @ "さび" if 1 < 0 => unreachable!(m),
        m @ "さび" if 1 > 0 => println!("guard conditional: {}", m),
        _ => unreachable!(),
    };

    // Slice pattern
    // Note, vector's can't be matched, only slices, hence the call to `as_slice()`
    match vec_int.as_slice() {
        [a, .., d] => println!("slice: {} {}", a, d),
        _ => unreachable!(),
    }

    // Tuple pattern
    vec_tuple.iter().for_each(|t| match t {
        (s, 1) => println!("tuple 1: {}", s),
        (s, 2) => println!("tuple 2: {}", s),
        (_, x) => println!("tuple x: {}", x),
    });

    // Structs
    vec_structs.iter().for_each(|color| match color {
        // Where `b` is between 200 -> 255
        Color(r, g, b @ 200..=255) => println!("r: {} g: {} b: {}", r, g, b),
        Color(0, 0, 0) => println!("black"),
        color => println!("reject: {:?}", color),
    });

    // Enums -- match each enum inside the vector
    vec_enums.iter().for_each(|e| match e {
        A::Player(x) => println!("player: {}", x),
        m @ A::Ready | m @ A::One => println!("{:?}", m),
    });

    // Enums -- match the entire vector, get only player numbers
    match vec_enums.as_slice() {
        [_, A::Player(x), A::Player(y), _] => println!("players: {} + {}", x, y),
        _ => unreachable!(),
    }

    // Destructuring

    let p1 = Palette {
        color: Color(30, 90, 43),
        selected: true,
    };

    let p2 = Palette {
        color: Color(4, 19, 79),
        selected: false,
    };

    let pp = vec![&p1, &p2];

    pp.iter().for_each(|p| match p {
        Palette {
            color: Color(r, g, b),
            selected: true,
        } => println!("intensity: {:.4}", (r + g + b) as f64 / (255 * 3) as f64),
        Palette {
            color,
            selected: false,
        } => println!("reject: {:?}", color),
    });

    // Destructure using `let`
    let Palette {
        color: Color(r, g, b),
        selected: ok,
    } = p1;
    println!("{}: {} {} {}", ok, r, g, b);

    // Destructuring function parameters
    f1(&p1.color);

    f2(&p2);

    let color = (0.4541, 0.3212, 0.5667);
    f3(&color);
}

fn f1(&Color(r, g, b): &Color) {
    println!("r: {} g: {} b: {}", r, g, b)
}

fn f2(
    &Palette {
        color: Color(r, g, b),
        selected, // shorthand field pattern
    }: &Palette,
) {
    if selected {
        println!("r: {} g: {} b: {}", r, g, b)
    } else {
        println!("r: {} g: {} b: {}", 0, 0, 0)
    }
}

fn f3(&(r, g, b): &(f64, f64, f64)) {
    println!("r: {} g: {} b: {}", r, g, b)
}
