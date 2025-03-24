```rust
// define a struct for person
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Declare an Option<Person> variable
    let person: Option<Person> = None;

    // Later, you can initialize it
    let person = Some(Person {
        name: String::from("Alice"),
        age: 30,
    });

    // Use the person variable
    if let Some(p) = person {
        println!("Name: {}, Age: {}", p.name, p.age);
    }
}

```

```kotlin
// define a struct
data class Person(
    val name: String,
    val age: Int
)

fun main() {
    // Declare a nullable Person variable
    var person: Person? = null

    // Later, you can initialize it
    person = Person("Alice", 30)

    // Use the person variable
    person?.let { p ->
        println("Name: ${p.name}, Age: ${p.age}")
    }
}

```
```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Define a struct for Person
struct Person {
    char *name;
    unsigned int age;
};

int main() {
    // Declare a pointer to a Person (representing Option<Person>)
    struct Person *person = NULL;

    // Later, you can initialize it
    person = malloc(sizeof(struct Person));
    if (person == NULL) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }

    // Initialize the person
    person->name = strdup("Alice"); // Allocate and copy the string
    person->age = 30;

    // Use the person variable
    if (person != NULL) {
        printf("Name: %s, Age: %u\n", person->name, person->age);
    }

    // Free allocated memory
    free(person->name); // Free the duplicated string
    free(person);       // Free the person struct

    return 0;
}

```
```golang
package main

import (
	"fmt"
)

// Define a struct for Person
type Person struct {
	Name string
	Age  uint32
}

func main() {
	// Declare a *Person variable (nil represents None)
	var person *Person

	// Later, you can initialize it
	person = &Person{
		Name: "Alice",
		Age:  30,
	}

	// Use the person variable
	if person != nil {
		fmt.Printf("Name: %s, Age: %d\n", person.Name, person.Age)
	}
}

```
```javascript
// Define a Person class
class Person {
    constructor(name, age) {
        this.name = name;
        this.age = age;
    }
}

function main() {
    // Declare a person variable initialized to null
    let person = null;

    // Later, you can initialize it
    person = new Person("Alice", 30);

    // Use the person variable
    if (person) {
        console.log(`Name: ${person.name}, Age: ${person.age}`);
    }
}

// Call the main function
main();

```
```python
class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age

def main():
    # Declare a variable that can hold a Person or None
    person = None

    # Later, you can initialize it
    person = Person(name="Alice", age=30)

    # Use the person variable
    if person is not None:
        print(f"Name: {person.name}, Age: {person.age}")

if __name__ == "__main__":
    main()

```
