#include <vector>

extern "C" {
    #include "benchmark.c"
}

void bubble_sort(std::vector<int> array) {
    benchmark();
    for (int i = 0; i < array.size(); i++) {
        for (int j = 0; j < array.size()-1; j++) {
            if (array[j] > array[j+1]) {
                // Swap:
                int temp = array[j];
                array[j] = array[j+1];
                array[j+1] = temp;
            }
        }
    }
    end_benchmark();
}