#include <stdio.h>

void simple() {
    printf("hello");
}

int withIf(int x) {
    if (x > 0) {
        return 1;
    } else {
        return 0;
    }
}

int withSwitch(int x) {
    switch (x) {
        case 1: return 1;
        case 2: return 2;
        default: return 0;
    }
}

void nested() {
    for (int i = 0; i < 10; i++) {
        if (i % 2 == 0) {
            continue;
        }
    }
}
