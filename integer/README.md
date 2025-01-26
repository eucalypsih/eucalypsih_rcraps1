```kotlin
/**
 * play.kotlinlang.org
 */

fun main(args: Array<String>) {

    // declares the variable x
    val x: Byte = Byte.MAX_VALUE
    
    // 127
    println(x)
}

```
```rust
fn main() {
    let  x: i8 = std::i8::MAX;

    // 127
    println!("{}", x)
}

```

```rust
fn main() {
    let x: i8 = std::i8::MIN;

    // -128
    println!("{}", x)
}

```

```kotlin
fun main(args: Array<String>) {
    val x: Byte = Byte.MIN_VALUE

    // -127
    println(x)
}

```

```kotlin
fun main(args: Array<String>) {
    val x: UByte = UByte.MAX_VALUE
    
    // 255
    println(x)
}

```

```kotlin
fun main(args: Array<String>) {
    val x: UByte = 255U
    
    // 255
    println(x)
}

```

```kotlin
fun main(args: Array<String>) {
    val x: Short = Short.MAX_VALUE
    
    // 32767
    println(x)
}

```

```kotlin
fun main(args: Array<String>) {
    val x: UShort = UShort.MAX_VALUE
    
    // 65535
    println(x)
}

```

```kotlin
fun main(args: Array<String>) {
    val x: Int = Int.MAX_VALUE
    
    // 2147483647
    println(x)
}

```

```kotlin
fun main(args: Array<String>) {
    val x: UInt = UInt.MAX_VALUE
    
    // 4294967295
    println(x)
}

```

```kotlin
fun main(args: Array<String>) {
    val x: Long = Long.MAX_VALUE
    
    // 9223372036854775807
    println(x)
}

```
```golang
package main

import (
    "fmt"
    "math"
)

func main() {
    // declares the variable x
    var x int64 = math.MaxInt64

    // 9223372036854775807
    fmt.Println(x)
}

```
```rust
fn main() {
    // declares the variable x
    let x: i64 = i64::MAX;
    
    // 9223372036854775807
    println!("{}", x);
}

```
```c
#include <stdio.h>
#include <limits.h>

int main(int argc, char *argv[]) {
    // declares the variable x
    long long x = LLONG_MAX;
    
    // 9223372036854775807
    printf("%lld\n", x);
    return 0;
}

```
bbai
```assembly
section .data
    msg db '9223372036854775807', 10  ; The number to print followed by a newline
    len equ $ - msg                    ; Length of the message

section .text
    global _start                      ; Entry point for the program

_start:
    ; Write the message to stdout
    mov rax, 1                         ; syscall: write
    mov rdi, 1                         ; file descriptor: stdout
    mov rsi, msg                       ; pointer to the message
    mov rdx, len                       ; length of the message
    syscall                            ; invoke the syscall

    ; Exit the program
    mov rax, 60                        ; syscall: exit
    xor rdi, rdi                       ; exit code: 0
    syscall                            ; invoke the syscall

```
```assembly
    .section .data
msg:    .asciz "9223372036854775807\n"  ; The number to print followed by a newline

    .section .text
    .global _start

_start:
    ; Write the message to stdout
    ldr r0, =1                        ; file descriptor: stdout
    ldr r1, =msg                      ; pointer to the message
    ldr r2, =len                      ; length of the message
    mov r7, #4                        ; syscall: write (syscall number 4)
    svc 0                             ; invoke the syscall

    ; Exit the program
    mov r7, #1                        ; syscall: exit (syscall number 1)
    mov r0, #0                        ; exit code: 0
    svc 0                             ; invoke the syscall

len = . - msg                        ; Calculate the length of the message

```
```kotlin
fun main(args: Array<String>) {

    // declares the variable x
    val x: ULong = ULong.MAX_VALUE
    
    // 18446744073709551615
    println(x)
}

```
```assembly

```
