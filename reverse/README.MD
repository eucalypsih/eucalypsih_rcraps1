```javascript
let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
arr.reverse();
console.log(arr);

```
```typescript
let arr: number[] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
arr.reverse();
console.log(arr);

```
```golang
package main

import (
	"fmt"
)

func main() {
	arr := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}
	for i, j := 0, len(arr)-1; i < j; i, j = i+1, j-1 {
		arr[i], arr[j] = arr[j], arr[i]
	}
	fmt.Println(arr)
}

```
```bash
clang -g -o rcraps_reverse_c rcraps_reverse_c.c
```
```c
#include <stdio.h>

int main() {
    int arr[] = {1, 2, 3, 4, 5, 6, 7, 8, 9};
    int len = sizeof(arr) / sizeof(arr[0]);
    for (int i = 0, j = len - 1; i < j; i++, j--) {
        int temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }
    for (int i = 0; i < len; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
    return 0;
}

```
```c++
#include <iostream>
#include <vector>

int main() {
    std::vector<int> arr = {1, 2, 3, 4, 5, 6, 7, 8, 9};
    for (int i = 0, j = arr.size() - 1; i < j; i++, j--) {
        std::swap(arr[i], arr[j]);
    }
    for (const auto& num : arr) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    return 0;
}

```
```rust
fn main() {
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut i = 0;
    let mut j = arr.len() - 1;
    while i < j {
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
    println!("{:?}", arr);
}

```
```kotlin
fun main() {
    val arr = mutableListOf(1, 2, 3, 4, 5, 6, 7, 8, 9)
    var i = 0
    var j = arr.size - 1
    while (i < j) {
        val temp = arr[i]
        arr[i] = arr[j]
        arr[j] = temp
        i++
        j--
    }
    println(arr)
}

```
```java
import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[] arr = {1, 2, 3, 4, 5, 6, 7, 8, 9};
        for (int i = 0, j = arr.length - 1; i < j; i++, j--) {
            int temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        }
        System.out.println(Arrays.toString(arr));
    }
}

```
```php
<?php

$arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
$i = 0;
$j = count($arr) - 1;

while ($i < $j) {
    $temp = $arr[$i];
    $arr[$i] = $arr[$j];
    $arr[$j] = $temp;
    $i++;
    $j--;
}

print_r($arr);
?>

```
```assembly
section .data
    arr db 1, 2, 3, 4, 5, 6, 7, 8, 9
    arr_len equ 9

section .bss
    i resb 1
    j resb 1

section .text
    global _start

_start:
    mov byte [i], 0
    mov byte [j], arr_len - 1

reverse_loop:
    movzx rax, byte [i]
    movzx rbx, byte [j]
    cmp rax, rbx
    jge print_array

    ; Swap arr[i] and arr[j]
    mov al, [arr + rax]
    mov bl, [arr + rbx]
    mov [arr + rax], bl
    mov [arr + rbx], al

    ; Increment i and decrement j
    inc byte [i]
    dec byte [j]
    jmp reverse_loop

print_array:
    ; Print the array (this part is simplified, actual printing would require syscall)
    ; For demonstration purposes, we will just exit
    mov rax, 60         ; syscall: exit
    xor rdi, rdi        ; status: 0
    syscall

```
