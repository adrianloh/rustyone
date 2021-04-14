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
    let csv = env::args()
        .nth(1)
        .expect("\n\nrequired path to csv file\n\n");
    let re_float = Regex::new(r"^\d\.\d+$").unwrap();
    let re_integer = Regex::new(r"^\d+$").unwrap();

    let reader = match File::open(&csv) {
        Ok(file) => BufReader::new(file),
        Err(err) => {
            eprintln!("cannot open file: {:?} ==> {}", err, &csv);
            exit(1);
        }
    };

    let mut parsed_lines: Vec<Row> = Vec::new();

    for _line in reader.lines() {
        let line = _line.unwrap();
        let cols: Row = line
            .split(',')
            .map(|l| l.to_string())
            .map(|s| match s {
                ref s if re_float.is_match(s) => Any::Float(s.parse::<f64>().unwrap()),
                ref s if re_integer.is_match(s) => Any::Integer(s.parse::<i64>().unwrap()),
                _ => Any::Str(s),
            })
            .collect();
        println!("{:?}", &cols);
        parsed_lines.push(cols);
    }
    println!("{}", parsed_lines.len());
}
