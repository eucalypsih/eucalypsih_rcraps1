GOLANG
```golang
package main

import (
	"fmt"
)

func main() {
	// Original string (in this case, a Hangul string)
	originalString := "액션" // "Action" in Korean

	// Convert the string to a byte slice (UTF-8 encoding)
	utf8Bytes := []byte(originalString)

	// Print the byte slice
        // UTF-8 Encoded Bytes:  [236 149 161 236 133 152]
	fmt.Println("UTF-8 Encoded Bytes: ", utf8Bytes)

	// Print the hexadecimal representation of the bytes
        // Hexadecimal Representation: ec95a1ec8598
	fmt.Printf("Hexadecimal Representation: %x\n", utf8Bytes)

	// Decode the byte slice back to a string
	decodedString := string(utf8Bytes)

	// Print the decoded string
        // Decoded String: 액션
	fmt.Println("Decoded String:", decodedString)

	// Check if the original string and decoded string are the same
        // The original and decoded strings are the same.
	if originalString == decodedString {
		fmt.Println("The original and decoded strings are the same.")
	} else {
		fmt.Println("The original and decoded strings are different.")
	}
}

```

RUST
```rust
fn main() {
    // Original string (in this case, a Hangul string)
    let original_string = "액션"; // "Action" in Korean

    // Convert the string to a byte vector (UTF-8 encoding)
    let utf8_bytes: &[u8] = original_string.as_bytes();

    // Print the byte vector
    // UTF-8 Encoded Bytes: [236, 149, 161, 236, 133, 152]
    println!("UTF-8 Encoded Bytes: {:?}", utf8_bytes);

    // Print the hexadecimal representation of the bytes
    // Hexadecimal Representation: ec95a1ec8598
    print!("Hexadecimal Representation: ");
    for byte in utf8_bytes {
        print!("{:02x}", byte);
    }
    println!();

    // Decode the byte vector back to a string
    // Decoded String: 액션
    let decoded_string = String::from_utf8_lossy(utf8_bytes);

    // Print the decoded string
    println!("Decoded String: {}", decoded_string);

    // Check if the original string and decoded string are the same
    // The original and decoded strings are the same.
    if original_string == decoded_string {
        println!("The original and decoded strings are the same.");
    } else {
        println!("The original and decoded strings are different.");
    }
}

```

C
```c
#include <stdio.h>
#include <string.h>
#include <locale.h>
#include <wchar.h>

int main() {
    // Set the locale to support wide characters
    setlocale(LC_CTYPE, "");

    // Original string (in this case, a Hangul string)
    wchar_t original_string[] = L"액션"; // "Action" in Korean

    // Convert the string to a byte array (UTF-8 encoding)
    // We need to calculate the size of the UTF-8 encoded string
    char utf8_bytes[256]; // Buffer to hold the UTF-8 bytes
    int len = wcstombs(utf8_bytes, original_string, sizeof(utf8_bytes));

    // Print the byte vector
    printf("UTF-8 Encoded Bytes: ");
    for (int i = 0; i < len; i++) {
        printf("%d ", (unsigned char)utf8_bytes[i]);
    }
    printf("\n");

    // Print the hexadecimal representation of the bytes
    printf("Hexadecimal Representation: ");
    for (int i = 0; i < len; i++) {
        printf("%02x", (unsigned char)utf8_bytes[i]);
    }
    printf("\n");

    // Decode the byte array back to a wide string
    wchar_t decoded_string[256];
    mbstowcs(decoded_string, utf8_bytes, sizeof(decoded_string) / sizeof(wchar_t));

    // Print the decoded string
    wprintf(L"Decoded String: %ls\n", decoded_string);

    // Check if the original string and decoded string are the same
    if (wcscmp(original_string, decoded_string) == 0) {
        printf("The original and decoded strings are the same.\n");
    } else {
        printf("The original and decoded strings are different.\n");
    }

    return 0;
}

```
