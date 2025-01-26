```kotlin
fun main() {
    val b: (UInt, UInt) -> UInt = { a, b -> (a + b) }
    println(b(4294967294.toUInt(), 1.toUInt())) // 4294967295 (2^32 - 1)
}

```
```rust
fn main() {
    let b: fn(u32, u32) -> u32 = |a: u32, b: u32| a + b;
    println!("{}", b(6u32, 3u32)); // 4294967295 (2^32 - 1)
}

```

```golang
package main

import (
	"fmt"
)

func main() {
	b := func(a, b uint32) uint32 {
		return a + b
	}
	fmt.Println(b(6, 3)) // 4294967295 (2^32 - 1)
}

```

```javascript
function main() {
    const b = (a, b) => a + b;
    console.log(b(6, 3));
}

main();

```

```typescript
function main() {
    const b: (a: number, b: number) => number = (a, b) => (a + b);
    console.log(b(4294967294, 1));
}

main();

```

```python
def main():
    b = lambda a, b: a + b
    print(b(4294967294, 1))

main()

```

```java
import java.util.function.BiFunction;

public class Main {
    public static void main(String[] args) {
        BiFunction<Integer, Integer, Integer> b = (a, b1) -> a + b1;
        System.out.println(b.apply(6, 3));
    }
}

```

```c++
#include <iostream>
#include <cstdint>

int main() {
    auto b = [](uint32_t a, uint32_t b) -> uint32_t {
        return a + b;
    };
    std::cout << b(4294967294u, 1u) << std::endl; // 4294967295 (2^32 - 1)
    return 0;
}

```

```c
#include <stdio.h>
#include <stdint.h>

typedef uint32_t (*UIntFunction)(uint32_t, uint32_t);

uint32_t add(uint32_t a, uint32_t b) {
    return a + b;
}

int main() {
    UIntFunction b = add;
    printf("%u\n", b(4294967294U, 1U)); // 4294967295 (2^32 - 1)
    return 0;
}

```

```lua
local b = function(a, b)
    return a + b
end

print(b(4294967294, 1))

```
