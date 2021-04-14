## Pattern matching

```rust
fn getResult() -> Result<i64> {
    if no_problemo {
        Ok(100)
    } else {
        SomeError
    }
}
```

To access the result, we have to `unwrap` it first:

```rust
let x = getResult().unwrap()
```

But this will panic if an `Err` was returned (which may be what we want).

```rust
let x = getResult()?
``` 






```rust
let x = match getResult() {
    Ok(t) => t,
    Err(e) => e
}
```

To get a simple default value in the event of an error:

```rust
let x = match getResult().unwrap_or(0)
```