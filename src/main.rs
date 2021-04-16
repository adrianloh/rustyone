struct C;

trait Multi<T> {
    fn m();
}

impl Multi<String> for C {
    fn m() {
        unimplemented!()
    }
}

fn main() {
    C::m();
}
