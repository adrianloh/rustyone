use custom_error::custom_error;

custom_error! {MyError
    Bad{when: String} = "fuck {when}",
    Terrible          = "die"
}

fn main() -> Result<(), MyError> {
    for i in 0..=2 {
        println!("Let's {}", match suredie(i) {
            Ok(s) => s,
            Err(e) => e.to_string()
        });
    }
    let d = suredie(1)?;
    println!("We will never get here {}", d);
    Ok(())
}

fn suredie(x: i32) -> Result<String, MyError> {
    let s = "eat ice cream".to_owned();
    match x {
        0 => Ok(s),
        1 => Err(MyError::Bad { when: "now".to_owned() }),
        _ => Err(MyError::Terrible)
    }
}
