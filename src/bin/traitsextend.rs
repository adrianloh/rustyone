use chrono::offset::Local;
use chrono::DateTime;
use std::time::SystemTime;

// Add two new methods to "SystemTime"
trait A {
    fn log_time(self) -> String;
    fn unix_nano(self) -> u128;
}

impl A for SystemTime {
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

// Add a method to "SystemTime" that returns a value based on its assigned type
trait B<T> {
    fn formatted(self) -> T;
}

impl B<u128> for SystemTime {
    fn formatted(self) -> u128 {
        self.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}

impl B<String> for SystemTime {
    fn formatted(self) -> String {
        let dt: DateTime<Local> = self.into();
        dt.format("[%d %b %Y %T]").to_string()
    }
}

// `A` and `B` extends `SystemTime` to do exactly the same thing.
// They differ only in how we use it in code
fn main() {
    let now = SystemTime::now();

    let x = now.log_time();
    let y = now.unix_nano();

    println!("{:?}", x);
    println!("{:?}", y);

    let s: String = now.formatted();
    let t: u128 = now.formatted();

    println!("{:?}", s);
    println!("{:?}", t);

    assert_eq!(x, s);
    assert_eq!(y, t);
}
