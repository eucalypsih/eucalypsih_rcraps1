```javascript
let arr = [8, 6, 3, 1, 2, 4, 7, 5, 9];
arr.sort();
console.log(arr);

```
```typescript
let arr: number[] = [8, 6, 3, 1, 2, 4, 7, 5, 9];
arr.sort((a, b) => a - b);
console.log(arr);

```
```php
<?php

$arr = [8, 6, 3, 1, 2, 4, 7, 5, 9];
sort($arr);
print_r($arr);

?>

```
```ruby
arr = [8, 6, 3, 1, 2, 4, 7, 5, 9]
arr.sort!
puts arr

```
```golang
package main

import (
	"fmt"
	"sort"
)

func main() {
	arr := []int{8, 6, 3, 1, 2, 4, 7, 5, 9}
	sort.Ints(arr)
	fmt.Println(arr)
}

```
```c
#include <stdio.h>
#include <stdlib.h>

int compare(const void *a, const void *b) {
    return (*(int*)a - *(int*)b);
}

int main() {
    int arr[] = {8, 6, 3, 1, 2, 4, 7, 5, 9};
    int n = sizeof(arr) / sizeof(arr[0]);
    
    qsort(arr, n, sizeof(int), compare);
    
    for (int i = 0; i < n; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
    
    return 0;
}

```
```c++
#include <iostream>
#include <vector>
#include <algorithm>

int main() {
    std::vector<int> arr = {8, 6, 3, 1, 2, 4, 7, 5, 9};
    std::sort(arr.begin(), arr.end());
    for (const auto& num : arr) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    return 0;
}

```
```rust
fn main() {
    let mut arr = vec![8, 6, 3, 1, 2, 4, 7, 5, 9];
    arr.sort();
    println!("{:?}", arr);
}

```
```kotlin
//import kotlin.math.sort

fun main() {
    val arr = intArrayOf(8, 6, 3, 1, 2, 4, 7, 5, 9)
    arr.sort()
    println(arr.joinToString(", "))
}

```
```java
import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[] arr = {8, 6, 3, 1, 2, 4, 7, 5, 9};
        Arrays.sort(arr);
        System.out.println(Arrays.toString(arr));
    }
}

```

