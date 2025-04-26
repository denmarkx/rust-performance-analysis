#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define FILE_NAME "test/bubble_sort.txt"
#define RANDOM_MIN 1
#define RANDOM_MAX 100000
#define NUM_ITERATIONS 100

int main() {
    FILE *file;
    int i, j;
    int array_size;
    int array_size_min = RANDOM_MAX / 6;
    int array_size_max = RANDOM_MAX / 2;

    srand((unsigned int)time(NULL));

    file = fopen(FILE_NAME, "w");
    if (!file) {
        perror("Failed to open file");
        return 1;
    }

    for (i = 0; i < NUM_ITERATIONS; ++i) {
        array_size = array_size_min + rand() % (array_size_max - array_size_min + 1);
        fprintf(file, "%d", array_size);

        for (j = 0; j < array_size; ++j) {
            int value = RANDOM_MIN + rand() % (RANDOM_MAX - RANDOM_MIN);
            fprintf(file, " %d", value);
        }
        fprintf(file, "\n");
    }

    fclose(file);
    return 0;
}