use std::fs::File;
use std::io::{self, Write};

struct MyStruct {
    file: File,
}

impl MyStruct {
    // Constructor that creates an instance of MyStruct
    fn new(file_path: &str) -> io::Result<Self> {
        // Create the file and handle the Result
        let x = File::create(file_path)?;
        Ok(MyStruct { file: x })
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

