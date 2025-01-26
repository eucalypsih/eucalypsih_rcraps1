get_ref
```rust
struct Wrapper<T> {
    inner: T,
}

impl<T> Wrapper<T> {
    pub fn get_ref(&self) -> &T {
        &self.inner
    }
}

fn main() {
    let wrapper: Wrapper<i8> = Wrapper { inner: 127 };
    let value: &i8 = wrapper.get_ref(); // value is now 127
    // wrapper can no longer be used here
    println!("Value: {}", value);
}

```

```rust
struct Wrapper<T> {
    inner: T,
}

impl<T> Wrapper<T> {
    pub fn new(inner: T) -> Self {
        Wrapper { inner }
    }

    pub fn get_ref(&self) -> &T {
        &self.inner
    }
}

fn main() {
    let wrapper: Wrapper<i8> = Wrapper::new(127);
    let value: &i8 = wrapper.get_ref(); // value is now 127
    // wrapper can no longer be used here
    println!("Value: {}", value);
}

```

get_mut
```rust
struct Wrapper<T> {
    inner: T,
}

impl<T> Wrapper<T> {
    pub fn get_ref(&mut self) -> &mut T {
        &mut self.inner
    }
}

fn main() {
    let mut wrapper: Wrapper<i8> = Wrapper { inner: 127 };
    let value: &mut i8 = wrapper.get_ref(); // value is now 127
    // Modify the inner value through the mutable reference
    *value -= 1;
    // wrapper can no longer be used here
    println!("Value: {}", value);
}

```
