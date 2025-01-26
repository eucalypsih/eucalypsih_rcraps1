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
