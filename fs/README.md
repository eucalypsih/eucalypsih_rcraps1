```rust
use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Membuka file untuk menulis (dan membuatnya jika tidak ada)
    let file = OpenOptions::new()
        .write(true)      // Izin untuk menulis
        .create(true)     // Buat file jika tidak ada
        .truncate(true)   // Hapus isi file jika sudah ada
        .open("output.txt")?; // Ganti dengan nama file yang diinginkan

    // Menulis ke file
    let mut writer = io::BufWriter::new(file);
    writeln!(writer, "Hello, world!")?; // Menulis ke file

    Ok(())
}

```
OpenOptions::new(): Membuat instance baru dari `OpenOptions`.
write(true): Menentukan bahwa file akan dibuka untuk menulis.
create(true): Menentukan bahwa file harus dibuat jika tidak ada.
truncate(true): Menentukan bahwa isi file harus dihapus jika file sudah ada.
open("output.txt"): Membuka file dengan opsi yang telah ditentukan. Jika file tidak dapat dibuka, operator `?` akan mengembalikan kesalahan.

