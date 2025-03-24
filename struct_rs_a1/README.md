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
```rust
use std::fs::File;
use std::io::{self, Write};

struct MyStruct {
    file: File,
}

impl MyStruct {
    // Constructor that creates an instance of MyStruct
    fn new(file_path: &str) -> io::Result<Self> {
        // Create the file and handle the Result
        let file = File::create(file_path)?;
        Ok(MyStruct { file })
    }

    // Example method to write to the file
    fn write_to_file(&mut self, data: &[u8]) -> io::Result<()> {
        self.file.write_all(data)?;
        self.file.flush()?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    // Create an instance of MyStruct
    let mut my_struct: MyStruct = MyStruct::new("output.txt")?;

    // Write some data to the file
    my_struct.write_to_file(b"Hello, world!")?;

    Ok(())
}

```
```rust
use std::fs::File;
use std::io::{self, Read};
use std::error::Error;

fn read_file_contents(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_path).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(contents)
}

fn main() {
    match read_file_contents("output.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

```
