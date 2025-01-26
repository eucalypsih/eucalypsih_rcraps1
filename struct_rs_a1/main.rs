fn main() {
    // Contoh byte array yang valid UTF-8
    let valid_utf8: &[u8] = b"Hello, world!";
    
    // Mengonversi byte array menjadi &str
    match std::str::from_utf8(valid_utf8) {
        Ok(string) => println!("String: {}", string),
        Err(e) => println!("Error: {}", e),
    }

    // Contoh byte array yang tidak valid UTF-8
    let invalid_utf8: &[u8] = &[0, 159, 146, 150]; // Ini bukan UTF-8 yang valid

    // Mengonversi byte array menjadi &str
    match std::str::from_utf8(invalid_utf8) {
        Ok(string) => println!("String: {}", string),
        Err(e) => println!("Error: {}", e),
    }
}
