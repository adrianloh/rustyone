use chrono::offset::Local;
use chrono::{DateTime, Duration};
use dirs::home_dir;
use std::ops::Sub;
use std::path::PathBuf;
use walkdir::WalkDir;

fn get_root() -> Option<PathBuf> {
    let home = home_dir()?;
    let root = home.join("Avaxgirls");
    if root.is_dir() {
        Some(root)
    } else {
        None
    }
}

fn main() {
    let root: PathBuf = get_root().expect("\n\nCannot get root\n\n");
    let mut files = Vec::new();
    for entry in WalkDir::new(&root)
        // Optional, get only the immediete descendants of root
        .min_depth(1)
        .max_depth(1)
    {
        let entry /*fs::DirEntry*/  = entry.unwrap();
        let fifo  /*fs::Metadata*/  = entry.metadata().unwrap();
        let mtime /*time::SystemTime*/ = fifo.modified().unwrap();
        // To get millis since epoch:
        //      mtime.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        if fifo.is_dir() {
            files.push((entry, mtime));
        }
    }

    // Sort by `mtime` -- oldest entry comes first
    // To reverse, swap `a` and `b` in the sort closure
    files.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    // Get the newest entry's mtime
    let newest_mtime = files.last().unwrap().1;
    // Use `chrono::Duration` for convenience since it has `::days()`
    // `std::time::Duration` only has `::from_[secs|millis|nanos]()`!
    let lookbehind_duration = Duration::days(7).to_std().unwrap();
    let lookbehind_mtime = newest_mtime.sub(lookbehind_duration);

    files
        .iter()
        // Keep only entries newer than `lookbehind_mtime`
        .filter(|(_, ref_mtime)| ref_mtime > &lookbehind_mtime)
        // Pretty print
        .for_each(|(ref_entry, ref_mtime)| {
            let mtime = *ref_mtime;
            let dt: DateTime<Local> = mtime.into();
            println!(
                "[{date}] {path}",
                date = dt.format("%d %b %Y %T"),
                path = ref_entry.path().display(),
            );
        });

    println!("total entries: {}", files.len());
}
