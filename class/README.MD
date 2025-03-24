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
class Lamp {

    // property (data member)
    private boolean isOn = false;

    // member function
    public void turnOn() {
        isOn = true;
    }

    // member function
    public void turnOff() {
        isOn = false;
    }

    public void displayLightStatus(String lamp) {
        if (isOn) {
            System.out.println(lamp + " lamp is on.");
        } else {
            System.out.println(lamp + " lamp is off.");
        }
    }
}

public class Main {
    public static void main(String[] args) {

        Lamp l1 = new Lamp(); // create l1 object of Lamp class
        Lamp l2 = new Lamp(); // create l2 object of Lamp class

        l1.turnOn();
        l2.turnOff();

        l1.displayLightStatus("l1");
        l2.displayLightStatus("l2");
    }
}

```
```javascript
class Lamp {
    // property (data member)
    #isOn = false; // private field

    // member function
    turnOn() {
        this.#isOn = true;
    }

    // member function
    turnOff() {
        this.#isOn = false;
    }

    displayLightStatus(lamp) {
        if (this.#isOn) {
            console.log(`${lamp} lamp is on.`);
        } else {
            console.log(`${lamp} lamp is off.`);
        }
    }
}

function main() {
    const l1 = new Lamp(); // create l1 object of Lamp class
    const l2 = new Lamp(); // create l2 object of Lamp class

    l1.turnOn();
    l2.turnOff();

    l1.displayLightStatus("l1");
    l2.displayLightStatus("l2");
}

main();

```
```c++
#include <iostream>
#include <string>

class Lamp {
private:
    bool isOn = false; // property (data member)

public:
    // member function
    void turnOn() {
        isOn = true;
    }

    // member function
    void turnOff() {
        isOn = false;
    }

    void displayLightStatus(const std::string& lamp) {
        if (isOn)
            std::cout << lamp << " lamp is on." << std::endl;
        else
            std::cout << lamp << " lamp is off." << std::endl;
    }
};

int main() {
    Lamp l1; // create l1 object of Lamp class
    Lamp l2; // create l2 object of Lamp class

    l1.turnOn();
    l2.turnOff();

    l1.displayLightStatus("l1");
    l2.displayLightStatus("l2");

    return 0;
}

```
```rust
struct Lamp {
    is_on: bool,
}

impl Lamp {
    fn new() -> Self {
        Lamp { is_on: false }
    }

    fn turn_on(&mut self) {
        self.is_on = true;
    }

    fn turn_off(&mut self) {
        self.is_on = false;
    }

    fn display_light_status(&self, lamp: &str) {
        if self.is_on {
            println!("{} lamp is on.", lamp);
        } else {
            println!("{} lamp is off.", lamp);
        }
    }
}

fn main() {
    let mut l1 = Lamp::new(); // create l1 object of Lamp struct
    let mut l2 = Lamp::new(); // create l2 object of Lamp struct

    l1.turn_on();
    l2.turn_off();

    l1.display_light_status("l1");
    l2.display_light_status("l2");
}

```
```golang
package main

import "fmt"

type Lamp struct {
	isOn bool
}

func (l *Lamp) turnOn() {
	l.isOn = true
}

func (l *Lamp) turnOff() {
	l.isOn = false
}

func (l *Lamp) displayLightStatus(lamp string) {
	if l.isOn {
		fmt.Printf("%s lamp is on.\n", lamp)
	} else {
		fmt.Printf("%s lamp is off.\n", lamp)
	}
}

func main() {
	l1 := Lamp{} // create l1 object of Lamp struct
	l2 := Lamp{} // create l2 object of Lamp struct

	l1.turnOn()
	l2.turnOff()

	l1.displayLightStatus("l1")
	l2.displayLightStatus("l2")
}

```
```python
class Lamp:
    def __init__(self):
        # property (data member)
        self._is_on = False

    # member function
    def turn_on(self):
        self._is_on = True

    # member function
    def turn_off(self):
        self._is_on = False

    def display_light_status(self, lamp):
        if self._is_on:
            print(f"{lamp} lamp is on.")
        else:
            print(f"{lamp} lamp is off.")

if __name__ == "__main__":
    l1 = Lamp()  # create l1 object of Lamp class
    l2 = Lamp()  # create l2 object of Lamp class

    l1.turn_on()
    l2.turn_off()

    l1.display_light_status("l1")
    l2.display_light_status("l2")

```
```ruby
class Lamp
  # property (data member)
  def initialize
    @is_on = false
  end

  # member function
  def turn_on
    @is_on = true
  end

  # member function
  def turn_off
    @is_on = false
  end

  def display_light_status(lamp)
    if @is_on
      puts "#{lamp} lamp is on."
    else
      puts "#{lamp} lamp is off."
    end
  end
end

def main
  l1 = Lamp.new # create l1 object of Lamp class
  l2 = Lamp.new # create l2 object of Lamp class

  l1.turn_on
  l2.turn_off

  l1.display_light_status("l1")
  l2.display_light_status("l2")
end

main

```
```php
<?php

class Lamp {

    // property (data member)
    private $isOn = false;

    // member function
    public function turnOn() {
        $this->isOn = true;
    }

    // member function
    public function turnOff() {
        $this->isOn = false;
    }

    public function displayLightStatus($lamp) {
        if ($this->isOn) {
            echo "$lamp lamp is on.\n";
        } else {
            echo "$lamp lamp is off.\n";
        }
    }
}

function main() {
    $l1 = new Lamp(); // create l1 object of Lamp class
    $l2 = new Lamp(); // create l2 object of Lamp class

    $l1->turnOn();
    $l2->turnOff();

    $l1->displayLightStatus("l1");
    $l2->displayLightStatus("l2");
}

main();

```
```lua
Lamp = {}
Lamp.__index = Lamp

function Lamp:new()
    local obj = setmetatable({}, self)
    obj.isOn = false
    return obj
end

function Lamp:turnOn()
    self.isOn = true
end

function Lamp:turnOff()
    self.isOn = false
end

function Lamp:displayLightStatus(lamp)
    if self.isOn then
        print(lamp .. " lamp is on.")
    else
        print(lamp .. " lamp is off.")
    end
end

function main()
    local l1 = Lamp:new() -- create l1 object of Lamp class
    local l2 = Lamp:new() -- create l2 object of Lamp class

    l1:turnOn()
    l2:turnOff()

    l1:displayLightStatus("l1")
    l2:displayLightStatus("l2")
end

main()

```
```swift
class Lamp {

    // property (data member)
    private var isOn: Bool = false

    // member function
    func turnOn() {
        isOn = true
    }

    // member function
    func turnOff() {
        isOn = false
    }

    func displayLightStatus(lamp: String) {
        if isOn {
            print("\(lamp) lamp is on.")
        } else {
            print("\(lamp) lamp is off.")
        }
    }
}

func main() {
    let l1 = Lamp() // create l1 object of Lamp class
    let l2 = Lamp() // create l2 object of Lamp class

    l1.turnOn()
    l2.turnOff()

    l1.displayLightStatus(lamp: "l1")
    l2.displayLightStatus(lamp: "l2")
}

main()

```
```c
#include <stdio.h>
#include <stdbool.h>

typedef struct {
    bool isOn;
} Lamp;

void turnOn(Lamp* lamp) {
    lamp->isOn = true;
}

void turnOff(Lamp* lamp) {
    lamp->isOn = false;
}

void displayLightStatus(Lamp* lamp, const char* lampName) {
    if (lamp->isOn) {
        printf("%s lamp is on.\n", lampName);
    } else {
        printf("%s lamp is off.\n", lampName);
    }
}

int main() {
    Lamp l1; // create l1 object of Lamp
    Lamp l2; // create l2 object of Lamp

    turnOn(&l1);
    turnOff(&l2);

    displayLightStatus(&l1, "l1");
    displayLightStatus(&l2, "l2");

    return 0;
}

```
```assembly
section .data
    lampOn db " lamp is on.", 0
    lampOff db " lamp is off.", 0

section .bss
    isOn resb 1

section .text
    global _start

_start:
    ; Create l1 object of Lamp class
    mov byte [isOn], 1 ; l1 isOn = true

    ; Create l2 object of Lamp class
    mov byte [isOn + 1], 0 ; l2 isOn = false

    ; l1.displayLightStatus("l1")
    mov rdi, lampOn
    call displayLightStatus

    ; l2.displayLightStatus("l2")
    mov rdi, lampOff
    call displayLightStatus

    ; Exit program
    mov rax, 60
    xor rdi, rdi
    syscall

displayLightStatus:
    ; Check if lamp is on
    cmp byte [isOn], 1
    je .lamp_on
    ; If off
    mov rax, 1
    mov rdi, 1
    mov rsi, rdi
    mov rdx, lampOff
    syscall
    jmp .end

.lamp_on:
    ; If on
    mov rax, 1
    mov rdi, 1
    mov rsi, rdi
    mov rdx, lampOn
    syscall

.end:
    ret

```

