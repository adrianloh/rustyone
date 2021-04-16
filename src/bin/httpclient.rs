#![allow(dead_code)]

use ureq::Error;

const BETINA: &str = "https://gist.githubusercontent.com/adrianloh/d85483b8d561397d03adc89f30943dcc/raw/010913dd40b3c8556b149b91b0a5a486d6764cde/females.txt";

fn main() {
    match ureq::get("https://adrian.wtf/femaless.txt").call() {
        Ok(resp) => println!("{}", resp.into_string().unwrap()),
        Err(Error::Status(404, response)) => println!("{}", response.status_text()),
        Err(e) => println!("{:?}", e),
    };
}

fn get_names() -> Result<Vec<String>, ureq::Error> {
    let body = ureq::get(BETINA).call()?.into_string()?;
    Ok(body
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>())
}
