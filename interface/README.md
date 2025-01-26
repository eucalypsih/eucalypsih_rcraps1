```kotlin
import kotlin.math.PI

// Define the interface
interface Shape {
    fun area(): Double
    fun perimeter(): Double
}

// Circle class that implements the Shape interface
data class Circle(val radius: Double) : Shape {
    override fun area(): Double {
        return PI * radius * radius
    }

    override fun perimeter(): Double {
        return 2 * PI * radius
    }
}

// Rectangle class that implements the Shape interface
data class Rectangle(val length: Double, val width: Double) : Shape {
    override fun area(): Double {
        return length * width
    }

    override fun perimeter(): Double {
        return 2 * (length + width)
    }
}

// Main function to demonstrate the interface
fun main() {
    var s: Shape

    s = Circle(radius = 5.0)
    println("C Area: ${s.area()}")
    println("C Perimeter: ${s.perimeter()}")

    s = Rectangle(length = 4.0, width = 3.0)
    println("R Area: ${s.area()}")
    println("R Perimeter: ${s.perimeter()}")
}

```
