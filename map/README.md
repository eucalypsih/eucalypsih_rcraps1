```javascript
const object1 = {
  a: 'somestring',
  b: 42,
  c: false,
};

console.log(Object.values(object1));

```
```typescript
const object1: { a: string; b: number; c: boolean } = {
  a: 'somestring',
  b: 42,
  c: false,
};

console.log(Object.values(object1));

```
```php

```
```ruby

```
```golang
package main

import (
	"fmt"
)

type Object1 struct {
	A string
	B int
	C bool
}

func main() {
	object1 := Object1{
		A: "somestring",
		B: 42,
		C: false,
	}

	values := []interface{}{object1.A, object1.B, object1.C}
	fmt.Println(values)
}

```
```c
#include <stdio.h>

typedef struct {
    char* a;
    int b;
    int c; // using int to represent boolean
} Object1;

int main() {
    Object1 object1 = {
        "somestring",
        42,
        0 // false
    };

    printf("%s\n", object1.a);
    printf("%d\n", object1.b);
    printf("%d\n", object1.c);

    return 0;
}

```
```c++

```
```rust
fn main() {
    let object1 = {
        a: String::from("somestring"),
        b: 42,
        c: false,
    };

    let values: Vec<&dyn std::fmt::Debug> = vec![&object1.a, &object1.b, &object1.c];
    println!("{:?}", values);
}

struct Object {
    a: String,
    b: i32,
    c: bool,
}

```
```kotlin
class Lamp {

    // property (data member)
    private var isOn: Boolean = false

    // member function
    fun turnOn() {
        isOn = true
    }

    // member function
    fun turnOff() {
        isOn = false
    }

    fun displayLightStatus(lamp: String) {
        if (isOn == true)
            println("$lamp lamp is on.")
        else
            println("$lamp lamp is off.")
    }
}

fun main(args: Array<String>) {

    val l1 = Lamp() // create l1 object of Lamp class
    val l2 = Lamp() // create l2 object of Lamp class

    l1.turnOn()
    l2.turnOff()

    l1.displayLightStatus("l1")
    l2.displayLightStatus("l2")
}

```
```java
import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        Object1 object1 = new Object1("somestring", 42, false);
        System.out.println(Arrays.asList(object1.getValues()));
    }
}

class Object1 {
    private String a;
    private int b;
    private boolean c;

    public Object1(String a, int b, boolean c) {
        this.a = a;
        this.b = b;
        this.c = c;
    }

    public Object[] getValues() {
        return new Object[]{a, b, c};
    }
}

```
