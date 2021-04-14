use chrono::offset::Local;
use chrono::DateTime;
use dirs::home_dir;
use std::path::PathBuf;
use std::process::exit;
use walkdir::WalkDir;

fn main() {
    let root = match get_root() {
        None => {
            println!("Cannot get root");
            exit(1);
        }
        Some(root) => root,
    };
    let mut files = Vec::new();
    for entry in WalkDir::new(&root).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let fifo = entry.metadata().unwrap();
        let mtime = fifo.modified().unwrap();
        // .duration_since(SystemTime::UNIX_EPOCH)
        // .unwrap()
        // .as_millis();
        if fifo.is_dir() {
            files.push((entry, mtime));
        }
    }
    files.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    files.iter().for_each(|(entry, _mtime)| {
        let mtime = *_mtime;
        let dt: DateTime<Local> = mtime.into();
        println!(
            "[{date}] {path}",
            date = dt.format("%d %b %Y %T"),
            path = entry.path().display(),
        );
    });
    println!("total files: {}", files.len());
}

fn get_root() -> Option<PathBuf> {
    let home = home_dir()?;
    let root = home.join("Avaxgirls");
    if root.is_dir() {
        Some(root)
    } else {
        None
    }
}
