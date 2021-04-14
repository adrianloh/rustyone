# Rust

1. ### [Error handling](/src/bin/errorhandle.rs)

   - ways to `unwrap`
   - custom `Result` and `Err` with `anyhow`
1. ### [Pattern matching](src/bin/match.rs)

1. ### Structs

   - [Declare / Create](src/bin/struct_basic.rs)
   - [Sortable structs](src/bin/struct_sort.rs)

1. ### [Iterators and Map/Filter/Reduce](src/bin/mapreduce.rs)

1. ### [Closures](src/bin/closures.rs)

   - `Fn` and `FnMut` traits

1. ### [Reading, parsing file](src/bin/csvparse.rs)

   - Command line arguments
   - `Bufread`-ing a file and processing lines
   - `regex` matching
    
1. ### [Walking a directory](src/bin/oswalk.rs)

   - Getting `$HOME`
   - `stat`-ing files/folders
   -  Dealing with `time` and `chrono`

1. ### [Threading](src/bin/threads.rs)

1. ### Channels with [`crossbeam`](https://docs.rs/crossbeam/0.8.0/crossbeam/)

   - [Basic](src/bin/channels.rs) Fan-out/Fan-in
   - [`SeqQueue`](src/bin/queue.rs) and channels
   - [http and file io with channels](src/bin/channelretry.rs)

1. ### [async tasks with `tokio`](src/bin/tokiotasks.rs)

1. ### HTTP server / client
   - [Example](src/bin/asyncserver1.rs)
   - [Example](src/bin/asyncserver2.rs)
