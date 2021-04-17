# Rust

1. ### [Error handling](src/bin/errorhandle.rs)

   - ways to `unwrap`

1. ### [Pattern matching and destructuring](src/bin/match.rs)

1. ### Structs

   - [Declare / Create](src/bin/struct_basic.rs)
   - [Sortable structs](src/bin/struct_sort.rs)

1. ### [Iterators and Filter/Map/Reduce](src/bin/mapreduce.rs)

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

   - [Basic Fan-out/Fan-in](src/bin/channels.rs)
   - [`SeqQueue`](src/bin/queue.rs) and channels
   - [http and file io with channels](src/bin/channelretry.rs)

1. ### async

   - `tokio` [tasks](src/bin/tokiotasks.rs)
   - [http server + client](src/bin/asyncserver.rs) and `FuturesUnordered`
   
1. ### [Traits and generic functions](src/bin/traitsgenericfunc.rs)
   - [Extending types](src/bin/traitsextend.rs)
   - [Pseudo multiple dispatch](/src/bin/multidispatch.rs)
   - [Returning values based on assigned type](src/bin/traitscast.rs)