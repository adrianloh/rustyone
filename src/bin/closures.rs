fn main() {
    /* Passing closures */

    // a "self-contained" closure
    let c1 = || 1.0;
    let mut x = call_then_add(c1, 1.0);
    assert_eq!(x, 2.0);

    // `c2` captures `x` from the environment as readonly e.g. immutable ref
    let c2 /* Fn */ = |y: f64| x + y;
    let z = call_then_add_v2(c2, 1.0);
    assert_eq!(z, 3.0);

    // `c3` captures `x` and mutates it
    let c3 /* FnMut */ = |y| x += y;
    call_with_arg(c3, 10.0);
    assert_eq!(x, 12.0);

    /* Returning closures */

    let mut s = vec![x, z];
    let c4 = first_item_add(&s); // borrow immutable, returns `Fn`
    let t = c4(1.0);
    assert_eq!(t, 13.0);

    drop(c4); // `c4` is holding an immutable ref `&s`, drop it because we need to borrow mutable next

    {
        let mut c5 = first_item_add_assign(&mut s); // borrow mutable, returns an `FnMut`
        c5(1.0);
        // drop(c5) -- implied, because we're in a new scope (which was not at all neccessary)
    }
    // We must drop `c5` or this won't compile because `assert_eq!` needs to borrow immutable
    assert_eq!(s[0], 13.0);

    // Same as what we did in the block without the hassle of an intermediete var
    first_item_add_assign(&mut s)(10.0);
    assert_eq!(s[0], 23.0);
}

fn call_then_add(f: impl Fn() -> f64, x: f64) -> f64 {
    f() + x
}

// Same signature as `call_then_add` but using `where`
fn call_then_add_v2<SomeTrait>(f: SomeTrait, x: f64) -> f64
where
    SomeTrait: Fn(f64) -> f64,
{
    f(x)
}

fn call_with_arg(mut f: impl FnMut(f64), y: f64) {
    f(y)
}

fn first_item_add(v: &[f64]) -> impl Fn(f64) -> f64 + '_ {
    move |y| v[0] + y
}

fn first_item_add_assign(v: &mut [f64]) -> impl FnMut(f64) + '_ {
    move |y| v[0] += y
}
