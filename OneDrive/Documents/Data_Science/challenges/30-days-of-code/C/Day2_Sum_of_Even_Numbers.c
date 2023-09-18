#include <stdio.h>

int main() {
    int total = 0;
    for (int i = 2; i <= 100; i += 2) {
        total += i;
    }
    printf("Sum of even numbers from 2 to 100: %d\n", total);
    return 0;
}
