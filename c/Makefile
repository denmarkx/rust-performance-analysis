CFLAGS = -O3
GCC = "gcc-14"

quick_sort: sorting/quick_sort.c
	$(GCC) $(CFLAGS) -o quick_sort sorting/quick_sort.c -I.

insertion_sort: sorting/insertion_sort.c
	$(GCC) $(CFLAGS) -o insertion_sort sorting/insertion_sort.c -I.

bubble_sort: sorting/bubble_sort.c
	$(GCC) $(CFLAGS) -o bubble_sort sorting/bubble_sort.c -I.

matrix_mult: math/matrix_mult.c
	$(GCC) $(CFLAGS) -o matrix_mult math/matrix_mult.c -I.

math: matrix_mult
sorting: insertion_sort quick_sort bubble_sort
all: sorting math

clean:
	del *.exe
	del *.o
