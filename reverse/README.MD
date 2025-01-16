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
