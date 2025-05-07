#include <stdlib.h>
#include <stdint.h>
#include <time.h>
#include <assert.h>

/*
* Linear Congruential Generator
* 
* Naive implementation for the sole use of
* providing a seed and having it generate the same
* values independent of language.
*/
uint32_t lcg(uint32_t *seed) {
    *seed = (*seed * 1664525u + 1013904223u);
    return *seed;
}

// Util for returning random values between a, b for len c.
// If seed is a number greater than 0, the random number generator will be
// the LCG function and not the one from stdlib.
int* random_value(int a, int b, int c, uint32_t *seed, uint32_t o_seed) {
    assert(a < b);

    int *array = (int*)malloc(c * sizeof(int));
    for (int i = 0; i < c; i++) {
        if (o_seed > 0) {
            array[i] = lcg(seed) % (b + 1 - a) + a;
        } else {
            array[i] = rand() % (b + 1 - a) + a;
        }
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
*
*
* If seed is a number greater than 0, the random number generator will be
* the LCG function and not the one from stdlib.
*/

ArrayData random_value_set(int a, int b, int c, int rand_length, uint32_t *seed) {
    ArrayData data;

    if (rand_length != 0) {
        c = rand_length; 
    }

    int length = c;
    data.arr_size = c;

    // copy the seed:
    uint32_t o_seed = *seed;

    // Pre-determine our individual array lengths:
    int* count_arr = (int*)malloc(c * sizeof(int));
    for (int i = 0; i < c; i++) {
        // Random length:
        length = rand_length;
        if (rand_length == 0) {
            if (o_seed > 0) {
                length = lcg(seed) % (b - (b/2)) + (b/2);
            } else {
                length = rand() % (b - (b/2)) + (b/2);
            }
        }
        count_arr[i] = length;
    }

    // proper initialization of 2d array:
    int** array = (int**)malloc(c * sizeof(int*));
    int* sub_array = NULL;

    // get c number of arrays.
    for (int i = 0; i < c; i++) {
        // Get length:
        sub_array = random_value(a, b, count_arr[i], seed, o_seed);
        array[i] = sub_array;
    }

    data.array = array;
    data.count_arr = count_arr;
    return data;
}
