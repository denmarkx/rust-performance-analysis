#include <stdio.h>
#include "random_value.c"

int main() {
    int* array = random_value(0, 100, 5);
    free(array);
    return 0;
}
