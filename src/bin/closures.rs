fn main() {
    // a "self-contained" closure
    let c1 = || 1.0;
    let mut x = fa(c1, 1.0);
    assert_eq!(x, 2.0);

    // `c2` captures `x` from the environment as readonly
    let c2 = |y: f64| x + y; // impl `Fn`
    let z = fb(c2, 1.0);
    assert_eq!(z, 3.0); // `x` is untouched!

    // `c3` captures `x` and mutates it
    let c3 = |y| x += y; // impl `FnMut`
    fc(c3, 10.0);
    assert_eq!(x, 12.0);

    // Returning closures
    let mut s = vec![x, z];
    let c4 = ffa(&s); // borrow immutable, returns an `Fn`-able
    assert_eq!(c4(1.0), 13.0); // `s` is untouched
    drop(c4); // `c4` is holding an immutable ref, drop it because we need to borrow mutable next
    {
        let mut c5 = ffb(&mut s); // borrow mutable, returns an `FnMut`-able
        c5(1.0);
        // drop(c5) -- implied, because we're in a new scope
    }
    assert_eq!(s[0], 13.0); // You must drop `c5` or this won't compile because `assert_eq!` needs to borrow immutable

    // Call the returned closure without the hassle of an intermediete var which needs dropping
    ffb(&mut s)(10.0);
    assert_eq!(s[0], 23.0);
}

fn fa(f: impl Fn() -> f64, x: f64) -> f64 {
    f() + x
}

// Same signature as `fa` but using `where`
fn fb<SomeTrait>(f: SomeTrait, x: f64) -> f64
where
    SomeTrait: Fn(f64) -> f64,
{
    f(x)
}

fn fc(mut f: impl FnMut(f64), y: f64) {
    f(y)
}

fn ffa(v: &Vec<f64>) -> impl Fn(f64) -> f64 + '_ {
    move |y| v[0] + y
}

fn ffb(v: &mut Vec<f64>) -> impl FnMut(f64) + '_ {
    move |y| v[0] += y
}
