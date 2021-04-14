const BETINA: &str = "https://gist.githubusercontent.com/adrianloh/d85483b8d561397d03adc89f30943dcc/raw/010913dd40b3c8556b149b91b0a5a486d6764cde/females.txt";

fn main() {
    let a = get_names().expect("cannot get names");
    let mut b = vec![];
    let mut c = vec![];
    a.iter(/* &self */).for_each(|s| {
        b.push(s);
    });
    a.into_iter(/* self */)
        .for_each(|s| {
            c.push(s);
        });
}

fn get_names() -> Result<Vec<String>, ureq::Error> {
    let body = ureq::get(BETINA).call()?.into_string()?;
    Ok(body
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>())
}
