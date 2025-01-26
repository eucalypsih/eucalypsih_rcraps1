## dangling refereces
```c
#include <stdio.h>
#include <stdlib.h>

char* create_string() {
    char* str = (char*)malloc(20 * sizeof(char)); // Alokasikan memori
    if (str != NULL) {
        snprintf(str, 20, "Hello, C!");
    }
    return str; // Mengembalikan pointer yang valid
}

int main() {
    char* ptr = create_string();
    if (ptr != NULL) {
        printf("%s\n", ptr);
        free(ptr); // Jangan lupa untuk membebaskan memori
    }
    return 0;
}

```
```c
#include <stdio.h>
#include <stdlib.h>

typedef struct {
    char* name;
} Person;

Person* create_person(const char* name) {
    Person* p = (Person*)malloc(sizeof(Person)); // Alokasikan memori untuk struct
    if (p != NULL) {
        p->name = (char*)malloc(20 * sizeof(char)); // Alokasikan memori untuk nama
        if (p->name != NULL) {
            snprintf(p->name, 20, "%s", name);
        }
    }
    return p; // Mengembalikan pointer ke struct
}

void free_person(Person* p) {
    if (p != NULL) {
        free(p->name); // Bebaskan memori untuk nama
        free(p); // Bebaskan memori untuk struct
    }
}

int main() {
    Person* person = create_person("Alice");
    if (person != NULL) {
        printf("Nama: %s\n", person->name);
        free_person(person); // Bebaskan memori
    }
    return 0;
}

```
