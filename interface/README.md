```kotlin
import kotlin.math.PI

// Define the interface
interface Shape {
    fun area(): Double
    fun peri(): Double
}

// Circle class that implements the Shape interface
data class Circle(val radius: Double) : Shape {
    override fun area(): Double {
        return PI * radius * radius
    }

    override fun peri(): Double {
        return 2.0 * PI * radius
    }
}

// Rectangle class that implements the Shape interface
data class Rectangle(val length: Double, val width: Double) : Shape {
    override fun area(): Double {
        return length * width
    }

    override fun peri(): Double {
        return 2.0 * (length + width)
    }
}

// Main function to demonstrate the interface
fun main(args: Array<String>) {
    var s: Shape

    s = Circle(radius = 5.0)
    println("C Area: %.14f".format(s.area())) // (14)78.53981633974483
    println("C Peri: %.14f".format(s.peri())) // (14)31.41592653589793

    s = Rectangle(length = 4.0, width = 3.0)
    println("R Area: %.14f".format(s.area()))
    println("R Peri: %.14f".format(s.peri()))
}

```

```golang
package main

import (
	"fmt"
	"math"
)

// Shape interface
type Shape interface {
	Area() float64
	Peri() float64
}

// Circle struct that implements the Shape interface
type Circle struct {
	radius float64
}

func (c Circle) Area() float64 {
	return math.Pi * c.radius * c.radius
}

func (c Circle) Peri() float64 {
	return 2.0 * math.Pi * c.radius
}

// Rectangle struct that implements the Shape interface
type Rectangle struct {
	length, width float64
}

func (r Rectangle) Area() float64 {
	return r.length * r.width
}

func (r Rectangle) Peri() float64 {
	return 2.0 * (r.length + r.width)
}

// Main function to demonstrate the interface
func main() {
	var s Shape

	s = Circle{radius: 5.0}
	fmt.Printf("C Area: %.14f\n", s.Area()) // (14)78.53981633974483
	fmt.Printf("C Peri: %.14f\n", s.Peri()) // (14)31.41592653589793

	s = Rectangle{length: 4.0, width: 3.0}
	fmt.Printf("R Area: %.14f\n", s.Area())
	fmt.Printf("R Peri: %.14f\n", s.Peri())
}

```

```rust
use std::f64::consts::PI;

// Define the trait
trait Shape {
    fn area(&self) -> f64;
    fn peri(&self) -> f64;
}

// Circle struct that implements the Shape trait
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn peri(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

// Rectangle struct that implements the Shape trait
struct Rectangle {
    length: f64,
    width: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }

    fn peri(&self) -> f64 {
        2.0 * (self.length + self.width)
    }
}

// Main function to demonstrate the trait
fn main() {
    let mut s: Box<dyn Shape>;

    s = Box::new(Circle { radius: 5.0 });
    println!("C Area: {:.14}", s.area()); // (14)78.53981633974483
    println!("C Peri: {:.14}", s.peri()); // (14)31.41592653589793

    s = Box::new(Rectangle { length: 4.0, width: 3.0 });
    println!("R Area: {:.14}", s.area());
    println!("R Peri: {:.14}", s.peri());
}

```

```c
#include <stdio.h>
#include <math.h>

#define PI 3.14159265358979323846

// Define the interface
typedef struct Shape {
    double (*area)(void *self);
    double (*peri)(void *self);
} Shape;

// Circle class that implements the Shape interface
typedef struct {
    Shape shape;
    double radius;
} Circle;

double circle_area(void *self) {
    Circle *circle = (Circle *)self;
    return PI * circle->radius * circle->radius;
}

double circle_peri(void *self) {
    Circle *circle = (Circle *)self;
    return 2.0 * PI * circle->radius;
}

// Rectangle class that implements the Shape interface
typedef struct {
    Shape shape;
    double length;
    double width;
} Rectangle;

double rectangle_area(void *self) {
    Rectangle *rectangle = (Rectangle *)self;
    return rectangle->length * rectangle->width;
}

double rectangle_peri(void *self) {
    Rectangle *rectangle = (Rectangle *)self;
    return 2.0 * (rectangle->length + rectangle->width);
}

// Main function to demonstrate the interface
int main() {
    Circle circle = {{circle_area, circle_peri}, 5.0};
    Rectangle rectangle = {{rectangle_area, rectangle_peri}, 4.0, 3.0};

    Shape *s;

    s = (Shape *)&circle;
    printf("C Area: %.14f\n", s->area(s)); // (14)78.53981633974483
    printf("C Peri: %.14f\n", s->peri(s)); // (14)31.41592653589793

    s = (Shape *)&rectangle;
    printf("R Area: %.14f\n", s->area(s));
    printf("R Peri: %.14f\n", s->peri(s));

    return 0;
}

```

```javascript
const PI = Math.PI;

// Define the interface
class Shape {
    area() {
        throw new Error("Method 'area()' must be implemented.");
    }

    peri() {
        throw new Error("Method 'peri()' must be implemented.");
    }
}

// Circle class that implements the Shape interface
class Circle extends Shape {
    constructor(radius) {
        super();
        this.radius = radius;
    }

    area() {
        return PI * this.radius * this.radius;
    }

    peri() {
        return 2.0 * PI * this.radius;
    }
}

// Rectangle class that implements the Shape interface
class Rectangle extends Shape {
    constructor(length, width) {
        super();
        this.length = length;
        this.width = width;
    }

    area() {
        return this.length * this.width;
    }

    peri() {
        return 2.0 * (this.length + this.width);
    }
}

// Main function to demonstrate the interface
function main() {
    let s;

    s = new Circle(5.0);
    console.log(`C Area: ${s.area().toFixed(14)}`); // (14)78.53981633974483
    console.log(`C Peri: ${s.peri().toFixed(14)}`); // (14)31.41592653589793

    s = new Rectangle(4.0, 3.0);
    console.log(`R Area: ${s.area().toFixed(14)}`);
    console.log(`R Peri: ${s.peri().toFixed(14)}`);
}

main();

```

```python
import math

# Define the interface
class Shape:
    def area(self):
        pass

    def peri(self):
        pass

# Circle class that implements the Shape interface
class Circle(Shape):
    def __init__(self, radius):
        self.radius = radius

    def area(self):
        return math.pi * self.radius * self.radius

    def peri(self):
        return 2.0 * math.pi * self.radius

# Rectangle class that implements the Shape interface
class Rectangle(Shape):
    def __init__(self, length, width):
        self.length = length
        self.width = width

    def area(self):
        return self.length * self.width

    def peri(self):
        return 2.0 * (self.length + self.width)

# Main function to demonstrate the interface
if __name__ == "__main__":
    s = None

    s = Circle(radius=5.0)
    print("C Area: {:.14f}".format(s.area()))  # (14)78.53981633974483
    print("C Peri: {:.14f}".format(s.peri()))  # (14)31.41592653589793

    s = Rectangle(length=4.0, width=3.0)
    print("R Area: {:.14f}".format(s.area()))
    print("R Peri: {:.14f}".format(s.peri()))

```
