use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};
use std::process::exit;
use std::str::FromStr;

#[derive(Debug)]
struct Ip4(u8, u8, u8, u8);

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    active: bool,
    #[serde(deserialize_with = "ip_from_str")]
    ip: Ip4,
    #[serde(deserialize_with = "datetime_from_i64")]
    created: NaiveDateTime,
}

fn ip_from_str<'de, D>(deserializer: D) -> Result<Ip4, D::Error>
where
    D: Deserializer<'de>,
{
    let input_str: &str = Deserialize::deserialize(deserializer)?;
    let addr: Vec<_> = input_str
        .split('.')
        .map(|i| u8::from_str(i).unwrap())
        .collect();
    let ip = Ip4(addr[0], addr[1], addr[2], addr[3]);
    Ok(ip)
}

fn datetime_from_i64<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let unix_secs: i64 = Deserialize::deserialize(deserializer)?;
    let dt = NaiveDateTime::from_timestamp(unix_secs, 0);
    Ok(dt)
}

static API: &str = "https://api.mockaroo.com/api/8515e9a0?count=10&key=9645d580";

fn main() {
    match ureq::get(API).call() {
        Ok(resp) => resp.into_string().unwrap().lines().for_each(|line| {
            // If not doing error checking:
            // let user: User = serde_json::from_str(line).unwrap()
            if let Ok(user) = serde_json::from_str::<User>(line) {
                let dt = user.created.format("[%d %b %Y %T]").to_string();
                println!("{} {:?}", dt, user)
            } else {
                unimplemented!()
            }
        }),
        Err(ureq::Error::Status(404, response)) => {
            // Intercept a specific Error response code
            println!("{} {}", response.status(), response.status_text()); //=> 404 NotFound
            exit(1);
        }
        Err(e) => {
            // All other errors
            println!("{:?}", e);
            exit(1);
        }
    };
}
