#include <windows.h>

// XXX: this'll obviously break when using on osx

double get_time() {
    LARGE_INTEGER t, f;
    QueryPerformanceCounter(&t);
    QueryPerformanceFrequency(&f);
    return (double)t.QuadPart / (double)f.QuadPart;
}


static double start_time;
static double end_time;

/*
* Starts benchmarking timer.
*/
static inline void benchmark() {
    start_time = get_time();
}

/*
* Stops benchmark and spits out result.
*/
static inline void end_benchmark() {
    end_time = get_time();
    printf("Time Elapsed: %f\n", (end_time - start_time));
}
