use chrono::offset::Local;
use chrono::DateTime;
use std::time::SystemTime;

// A trait that extends a type -- in this case we're extending rust's `SystemTime`
trait LogTime {
    fn log_time(self) -> String;
    fn unix_nano(self) -> u128;
}

impl LogTime for SystemTime {
    // A slight gotcha, `SystemTime` implements `Copy`, so after these
    // methods are done, `self` is still around for the caller!
    fn log_time(self) -> String {
        let dt: DateTime<Local> = self.into();
        dt.format("[%d %b %Y %T]").to_string()
    }
    fn unix_nano(self) -> u128 {
        self.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}

fn main() {
    let now = SystemTime::now();
    println!("{} terminated", now.log_time());
    println!("{}", now.unix_nano());
}
