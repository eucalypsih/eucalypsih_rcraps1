Golang
```golang
package main

import (
    "fmt"
)

// Define an interface
type Animal interface {
    Speak() string
}

// Define a struct that implements the Animal interface
type Dog struct{}

func (d Dog) Speak() string {
    return "Woof!"
}

// Define another struct that implements the Animal interface
type Cat struct{}

func (c Cat) Speak() string {
    return "Meow!"
}

func main() {
    // Create a slice of Animal interface
    animals := []Animal{Dog{}, Cat{}}

    for _, animal := range animals {
        // Use type assertion to determine the concrete type
        switch a := animal.(type) {
        case Dog:
            fmt.Println("This is a Dog:", a.Speak())
        case Cat:
            fmt.Println("This is a Cat:", a.Speak())
        default:
            fmt.Println("Unknown animal")
        }
    }

    // Example of a type assertion with ok idiom
    var someAnimal Animal = Dog{}
    if dog, ok := someAnimal.(Dog); ok {
        fmt.Println("Successfully asserted to Dog:", dog.Speak())
    } else {
        fmt.Println("Failed to assert to Dog")
    }

    // Attempting to assert to a different type
    if cat, ok := someAnimal.(Cat); ok {
        fmt.Println("Successfully asserted to Cat:", cat.Speak())
    } else {
        fmt.Println("Failed to assert to Cat")
    }
}

```

Rust
```rust
// Define a trait
trait Animal {
    fn speak(&self) -> String;
}

// Define a struct that implements the Animal trait
struct Dog;

impl Animal for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
}

// Define another struct that implements the Animal trait
struct Cat;

impl Animal for Cat {
    fn speak(&self) -> String {
        "Meow!".to_string()
    }
}

fn main() {
    // Create a vector of Box<dyn Animal> to hold different types
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];

    for animal in &animals {
        // Use downcasting to determine the concrete type
        if let Some(dog) = animal.as_any().downcast_ref::<Dog>() {
            println!("This is a Dog: {}", dog.speak());
        } else if let Some(cat) = animal.as_any().downcast_ref::<Cat>() {
            println!("This is a Cat: {}", cat.speak());
        } else {
            println!("Unknown animal");
        }
    }

    // Example of a type assertion with a match statement
    let some_animal: Box<dyn Animal> = Box::new(Dog);
    if let Some(dog) = some_animal.as_any().downcast_ref::<Dog>() {
        println!("Successfully asserted to Dog: {}", dog.speak());
    } else {
        println!("Failed to assert to Dog");
    }

    // Attempting to assert to a different type
    if let Some(cat) = some_animal.as_any().downcast_ref::<Cat>() {
        println!("Successfully asserted to Cat: {}", cat.speak());
    } else {
        println!("Failed to assert to Cat");
    }
}

// Trait to allow downcasting
trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static + Animal> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

```

C
```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Define a struct for Animal
typedef struct Animal {
    void (*speak)(struct Animal*); // Function pointer for speak method
} Animal;

// Define a struct for Dog
typedef struct Dog {
    Animal base; // Base struct
} Dog;

// Define a struct for Cat
typedef struct Cat {
    Animal base; // Base struct
} Cat;

// Dog's speak function
void dog_speak(Animal* animal) {
    printf("This is a Dog: Woof!\n");
}

// Cat's speak function
void cat_speak(Animal* animal) {
    printf("This is a Cat: Meow!\n");
}

// Function to create a Dog
Dog* create_dog() {
    Dog* dog = (Dog*)malloc(sizeof(Dog));
    dog->base.speak = dog_speak; // Assign the speak function
    return dog;
}

// Function to create a Cat
Cat* create_cat() {
    Cat* cat = (Cat*)malloc(sizeof(Cat));
    cat->base.speak = cat_speak; // Assign the speak function
    return cat;
}

// Function to free the animal
void free_animal(Animal* animal) {
    free(animal);
}

int main() {
    // Create an array of Animal pointers
    Animal* animals[2];
    animals[0] = (Animal*)create_dog(); // Create a Dog
    animals[1] = (Animal*)create_cat(); // Create a Cat

    // Iterate over the animals and call their speak method
    for (int i = 0; i < 2; i++) {
        animals[i]->speak(animals[i]);
    }

    // Free the allocated memory
    for (int i = 0; i < 2; i++) {
        free_animal(animals[i]);
    }

    return 0;
}

```

Java
```java
// Define an interface
interface Animal {
    String speak();
}

// Define a class that implements the Animal interface
class Dog implements Animal {
    @Override
    public String speak() {
        return "Woof!";
    }
}

// Define another class that implements the Animal interface
class Cat implements Animal {
    @Override
    public String speak() {
        return "Meow!";
    }
}

public class Main {
    public static void main(String[] args) {
        // Create an array of Animal references
        Animal[] animals = new Animal[2];
        animals[0] = new Dog(); // Create a Dog
        animals[1] = new Cat(); // Create a Cat

        // Iterate over the animals and call their speak method
        for (Animal animal : animals) {
            if (animal instanceof Dog) {
                System.out.println("This is a Dog: " + animal.speak());
            } else if (animal instanceof Cat) {
                System.out.println("This is a Cat: " + animal.speak());
            } else {
                System.out.println("Unknown animal");
            }
        }

        // Example of a type assertion with instanceof
        Animal someAnimal = new Dog();
        if (someAnimal instanceof Dog) {
            Dog dog = (Dog) someAnimal;
            System.out.println("Successfully asserted to Dog: " + dog.speak());
        } else {
            System.out.println("Failed to assert to Dog");
        }

        // Attempting to assert to a different type
        if (someAnimal instanceof Cat) {
            Cat cat = (Cat) someAnimal;
            System.out.println("Successfully asserted to Cat: " + cat.speak());
        } else {
            System.out.println("Failed to assert to Cat");
        }
    }
}

```

Kotlin
```kotlin
// Define an interface
interface Animal {
    fun speak(): String
}

// Define a class that implements the Animal interface
class Dog : Animal {
    override fun speak(): String {
        return "Woof!"
    }
}

// Define another class that implements the Animal interface
class Cat : Animal {
    override fun speak(): String {
        return "Meow!"
    }
}

fun main() {
    // Create a list of Animal references
    val animals: List<Animal> = listOf(Dog(), Cat())

    // Iterate over the animals and call their speak method
    for (animal in animals) {
        when (animal) {
            is Dog -> println("This is a Dog: ${animal.speak()}")
            is Cat -> println("This is a Cat: ${animal.speak()}")
            else -> println("Unknown animal")
        }
    }

    // Example of a type assertion with smart casting
    val someAnimal: Animal = Dog()
    if (someAnimal is Dog) {
        println("Successfully asserted to Dog: ${someAnimal.speak()}")
    } else {
        println("Failed to assert to Dog")
    }

    // Attempting to assert to a different type
    if (someAnimal is Cat) {
        println("Successfully asserted to Cat: ${someAnimal.speak()}")
    } else {
        println("Failed to assert to Cat")
    }
}
l
```

Dart
```dart
// Define an abstract class (interface)
abstract class Animal {
  String speak();
}

// Define a class that implements the Animal interface
class Dog implements Animal {
  @override
  String speak() {
    return "Woof!";
  }
}

// Define another class that implements the Animal interface
class Cat implements Animal {
  @override
  String speak() {
    return "Meow!";
  }
}

void main() {
  // Create a list of Animal references
  List<Animal> animals = [Dog(), Cat()];

  // Iterate over the animals and call their speak method
  for (var animal in animals) {
    if (animal is Dog) {
      print("This is a Dog: ${animal.speak()}");
    } else if (animal is Cat) {
      print("This is a Cat: ${animal.speak()}");
    } else {
      print("Unknown animal");
    }
  }

  // Example of a type assertion with 'is'
  Animal someAnimal = Dog();
  if (someAnimal is Dog) {
    print("Successfully asserted to Dog: ${someAnimal.speak()}");
  } else {
    print("Failed to assert to Dog");
  }

  // Attempting to assert to a different type
  if (someAnimal is Cat) {
    print("Successfully asserted to Cat: ${someAnimal.speak()}");
  } else {
    print("Failed to assert to Cat");
  }
}

```

Python
```python
from abc import ABC, abstractmethod

# Define an abstract base class (interface)
class Animal(ABC):
    @abstractmethod
    def speak(self):
        pass

# Define a class that implements the Animal interface
class Dog(Animal):
    def speak(self):
        return "Woof!"

# Define another class that implements the Animal interface
class Cat(Animal):
    def speak(self):
        return "Meow!"

def main():
    # Create a list of Animal references
    animals = [Dog(), Cat()]

    # Iterate over the animals and call their speak method
    for animal in animals:
        if isinstance(animal, Dog):
            print(f"This is a Dog: {animal.speak()}")
        elif isinstance(animal, Cat):
            print(f"This is a Cat: {animal.speak()}")
        else:
            print("Unknown animal")

    # Example of a type assertion with isinstance
    some_animal = Dog()
    if isinstance(some_animal, Dog):
        print(f"Successfully asserted to Dog: {some_animal.speak()}")
    else:
        print("Failed to assert to Dog")

    # Attempting to assert to a different type
    if isinstance(some_animal, Cat):
        print(f"Successfully asserted to Cat: {some_animal.speak()}")
    else:
        print("Failed to assert to Cat")

# Run the main function
if __name__ == "__main__":
    main()

```

PHP
```php
<?php

// Define an interface
interface Animal {
    public function speak(): string;
}

// Define a class that implements the Animal interface
class Dog implements Animal {
    public function speak(): string {
        return "Woof!";
    }
}

// Define another class that implements the Animal interface
class Cat implements Animal {
    public function speak(): string {
        return "Meow!";
    }
}

// Main function
function main() {
    // Create an array of Animal references
    $animals = [new Dog(), new Cat()];

    // Iterate over the animals and call their speak method
    foreach ($animals as $animal) {
        if ($animal instanceof Dog) {
            echo "This is a Dog: " . $animal->speak() . PHP_EOL;
        } elseif ($animal instanceof Cat) {
            echo "This is a Cat: " . $animal->speak() . PHP_EOL;
        } else {
            echo "Unknown animal" . PHP_EOL;
        }
    }

    // Example of a type assertion with instanceof
    $someAnimal = new Dog();
    if ($someAnimal instanceof Dog) {
        echo "Successfully asserted to Dog: " . $someAnimal->speak() . PHP_EOL;
    } else {
        echo "Failed to assert to Dog" . PHP_EOL;
    }

    // Attempting to assert to a different type
    if ($someAnimal instanceof Cat) {
        echo "Successfully asserted to Cat: " . $someAnimal->speak() . PHP_EOL;
    } else {
        echo "Failed to assert to Cat" . PHP_EOL;
    }
}

// Run the main function
main();

?>

```

JavaScript
```javascript
// Define an interface-like structure using classes
class Animal {
    speak() {
        throw new Error("Method 'speak()' must be implemented.");
    }
}

// Define a Dog class that extends Animal
class Dog extends Animal {
    speak() {
        return "Woof!";
    }
}

// Define a Cat class that extends Animal
class Cat extends Animal {
    speak() {
        return "Meow!";
    }
}

// Main function
function main() {
    // Create an array of Animal instances
    const animals = [new Dog(), new Cat()];

    // Iterate over the animals and call their speak method
    animals.forEach(animal => {
        if (animal instanceof Dog) {
            console.log("This is a Dog: " + animal.speak());
        } else if (animal instanceof Cat) {
            console.log("This is a Cat: " + animal.speak());
        } else {
            console.log("Unknown animal");
        }
    });

    // Example of type checking
    const someAnimal = new Dog();
    if (someAnimal instanceof Dog) {
        console.log("Successfully asserted to Dog: " + someAnimal.speak());
    } else {
        console.log("Failed to assert to Dog");
    }

    // Attempting to assert to a different type
    if (someAnimal instanceof Cat) {
        console.log("Successfully asserted to Cat: " + someAnimal.speak());
    } else {
        console.log("Failed to assert to Cat");
    }
}

// Run the main function
main();

```
