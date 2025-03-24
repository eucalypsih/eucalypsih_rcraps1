```rust
// Define a struct
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Create an instance of the struct
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Create a reference to the instance
    let person_ref = &person;

    // Access the fields of the struct through the reference
    println!("Name: {}, Age: {}", person_ref.name, person_ref.age);
}

```
```kotlin
// Define a data class
data class Person(
    val name: String,
    val age: Int
)

fun main() {
    // Create an instance of the data class
    val person = Person(
        name = "Alice",
        age = 30
    )

    // Create a reference to the instance (not necessary in Kotlin, but for clarity)
    val personRef = person

    // Access the fields of the data class through the reference
    println("Name: ${personRef.name}, Age: ${personRef.age}")
}

```
```python
# Define a class
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age

def main():
    # Create an instance of the class
    person = Person("Alice", 30)

    # Access the fields of the class instance
    print(f"Name: {person.name}, Age: {person.age}")

if __name__ == "__main__":
    main()

```
