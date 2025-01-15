```text
ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQDBz8tU7e7ztS2Tc
O32OdjlVipq3U1yeaHy3Kx4eWDi6HKqzRXqjRXyekdcUMH8CVmqJY
LyLScCGRgH5VBzLSweB9vKvDFq/W6bY5Lef1oJE1Pr6Wg0sVrP1ao
2byROjI9JKNU5mbF0Z1tFlXiBcpAcbhzjQ5MWMA5K65CXX8WG99KA
SllL74HifR9OGAW2y9hKSfmhH33zqMIWIO72VCstx0DBjz1C9Hl1e
Y9OLA6CO0gHRL0cueT55lVnw+xuPhZE9A6i//lK4QRAjCihv11Kvb
5k0b6hECLreSfRASbNw0rsAaRc7tCNRDx+W+O1L/7hbuzhyp4nSGn
oZbsMNmqCQX7A7KCxeovrA+5Vwx5VNmd0TmUudddMcP4z4SSsYc1T
2rMmGPq7ndmcD97f5G98IIRYWuk9QqHkvuE3145zRyufzdS68oGvI
EsWZfmdKl95rTf9qyCnM1lzPw/kFCzkbeyiofZI5RDpr4p07Lzrgd
wqjzyT4W/N5+6cnRe+i9qUlkv4aHre32cenaV7tJGCGVQ91vcg2js
ES2nDxtE8UQcOMPUUKDVtN7+TpF/XJZyeT9f+Dg6DYzucVQrLUBq8
oNAWkC9EJFQjuY2+mMvY2RHgO7v/sImoCYOnDCYmqOZIybsGp93Gq
UG4DCq3raaGkjZeeKBsHO8i67h3QF+vqQ== eucalypsih@gmail.
com
```
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

