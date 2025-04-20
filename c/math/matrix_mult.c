#include <stdio.h>

#include "random_value.c"
#include "benchmark.c"

void matrix_mult(ArrayData mat_A, ArrayData mat_B) {
    benchmark();
    int count = mat_A.arr_size;
    int (*array)[count] = malloc(sizeof(int[count][count]));
    for (int i = 0; i < count; i++) {
        for (int j = 0; j < count; j++) {
            array[i][j] = 0;
            for (int k = 0; k < count; k++) {
                array[i][j] += mat_A.array[i][k] * mat_B.array[k][j];
            }
        }
    }
    end_benchmark();
}

int main(int argc, char* argv[]) {
    int n_iterations = atoi(argv[1]);
    setup_benchmark(n_iterations);

    ArrayData mat_A = random_value_set(0, 100, n_iterations, n_iterations);
    ArrayData mat_B = random_value_set(0, 100, n_iterations, n_iterations);
    matrix_mult(mat_A, mat_B);

    complete_benchmark();
    return 0;
}
