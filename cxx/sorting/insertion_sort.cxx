#include <vector>

extern "C" {
    #include "benchmark.c"
}

void insertion_sort(std::vector<int> array) {
    benchmark();

    for (size_t i = 1; i < array.size(); i++) {
        int n = array[i];
        int j = i-1;

        while (j >= 0 && array[j] > n) {
            array[j+1] = array[j];
            j--;
        }

        array[j+1] = n;
    }

    end_benchmark();
}
