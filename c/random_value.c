#include <stdlib.h>
#include <time.h>
#include <assert.h>

// Util for returning random values between a, b for len c.
int* random_value(int a, int b, int c) {
    assert(a < b);
    srand(time(NULL));

    int *array = malloc(c * sizeof(int));
    for (int i = 0; i < c; i++) {
        array[i] = rand() % (b + 1 - a) + a;
    }

    return array;
}
