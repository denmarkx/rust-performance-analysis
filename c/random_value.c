#include <stdlib.h>
#include <time.h>
#include <assert.h>

#include "util.c"

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

/*
* Returns ArrayData of random_value arrays.
* random_value range is [a, b] with a length of random [b/2, b].
* the number of iterations within arrayData's array is c.
*/

ArrayData random_value_set(int a, int b, int c) {
    ArrayData data;
    data.arr_size = c;

    int length = 2;

    // Pre-determine our individual array lengths:
    int* count_arr = malloc(c * sizeof(int));
    for (int i = 0; i < c; i++) {
        // Random length:
        length = rand() % (b + 1 - (b/2)) + (b/2);
        count_arr[i] = length;
    }

    // proper initialization of 2d array:
    int** array = malloc(sizeof(*array) * c);
    int* sub_array = NULL;

    // get c number of arrays.
    for (int i = 0; i < c; i++) {
        // Get length:
        sub_array = random_value(a, b, count_arr[i]);
        array[i] = sub_array;
        int* test = array[i];
        // for (int j = 0; j < count_arr[i]; j++) {
            // printf("array[j]=%d\n", test[j]);
        // }
    }

    data.array = array;
    data.count_arr = count_arr;
    return data;
}
