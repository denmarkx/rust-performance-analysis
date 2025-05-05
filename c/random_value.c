#include <stdlib.h>
#include <time.h>
#include <assert.h>

// Util for returning random values between a, b for len c.
int* random_value(int a, int b, int c) {
    assert(a < b);

    int *array = (int*)malloc(c * sizeof(int));
    for (int i = 0; i < c; i++) {
        array[i] = rand() % (b + 1 - a) + a;
    }

    return array;
}

/*
* Given three parameters, a, b, c, returns a 2D vector of randomized values.
* a (u32), b (u32) -- random_range(a..b+1) for each element in inner vector.
* c (usize): Number of arrays.
*
* The length of the inner array is determined by:
*   [0..random_range(b / 2..=b)].
*   ..IF inner_length is 0.
*   ..otherwise, it will use whatever was passed to inner_length
*/

ArrayData random_value_set(int a, int b, int c, int rand_length) {
    srand(time(NULL));
    ArrayData data;

    if (rand_length != 0) {
        c = rand_length; 
    }

    int length = c;
    data.arr_size = c;

    // Pre-determine our individual array lengths:
    int* count_arr = (int*)malloc(c * sizeof(int));
    for (int i = 0; i < c; i++) {
        // Random length:
        length = rand_length;
        if (rand_length == 0) {
            length = rand() % (b + 1 - (b/2)) + (b/2);
        }
        count_arr[i] = length;
    }

    // proper initialization of 2d array:
    int** array = (int**)malloc(c * sizeof(int*));
    int* sub_array = NULL;

    // get c number of arrays.
    for (int i = 0; i < c; i++) {
        // Get length:
        sub_array = random_value(a, b, count_arr[i]);
        array[i] = sub_array;
    }

    data.array = array;
    data.count_arr = count_arr;
    return data;
}
