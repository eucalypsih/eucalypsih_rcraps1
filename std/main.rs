struct Wrapper<T> {
    inner: T,
}

impl<T> Wrapper<T> {
    fn into_inner(self) -> T {
        self.inner
    }
}

fn main() {
    let wrapper: Wrapper<i32> = Wrapper { inner: 42 };
    let value: i32 = wrapper.into_inner(); // value is now 42
    println!("Value: {}", value);
}
