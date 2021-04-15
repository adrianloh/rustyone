use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

#[derive(Debug)]
enum Any {
    Str(String),
    Float(f64),
    Integer(i64),
}

type Row = Vec<Any>;

fn main() {
    // Get `$1` passed to the program, panic if nothing was given
    let csv = env::args()
        .nth(1)
        .expect("\n\nrequired path to csv file\n\n");
    let re_float = Regex::new(r"^\d\.\d+$").unwrap();
    let re_integer = Regex::new(r"^\d+$").unwrap();

    // Create a buff reader for the file, `exit(1)` if anything goes wrong
    let reader = match File::open(&csv) {
        Ok(file) => BufReader::new(file),
        Err(err) => {
            eprintln!("cannot open file: {:?} ==> {}", err, &csv);
            exit(1);
        }
    };

    let mut parsed_lines: Vec<Row> = Vec::new();

    // For each line in the buffer
    for _line in reader.lines() {
        // This gives us a comma-delimited line
        let line = _line.unwrap();
        // Split each line on commas, try and cast each "column" into a value
        let row: Row = line
            .split(',') // `split(String)` => [&str, &str, &str]
            .map(|l| l.to_string())
            .map(|s| match s {
                _ if re_float.is_match(&s) => Any::Float(s.parse::<f64>().unwrap()),
                _ if re_integer.is_match(&s) => Any::Integer(s.parse::<i64>().unwrap()),
                _ => Any::Str(s),
            })
            .collect();
        println!("{:?}", &row);
        parsed_lines.push(row);
    }
    println!("total lines: {}", parsed_lines.len());

    // Up to this point, we've naively parsed any csv with an arbitrary
    // number of columns. But now we'd like to check if a row matches
    // our "template", and if it does, extract its values

    let area = match parsed_lines[0].as_slice() {
        // where the row is string, int, int
        [_, Any::Integer(x), Any::Integer(y)] => x * y,
        _ => 0,
    };
    println!("area: {}", area);
}
