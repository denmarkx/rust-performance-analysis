CFLAGS = -O3

quick_sort: sorting/quick_sort.c
	gcc $(CFLAGS) -o quick_sort sorting/quick_sort.c -I.

insertion_sort: sorting/insertion_sort.c
	gcc $(CFLAGS) -o insertion_sort sorting/insertion_sort.c -I.

bubble_sort: sorting/bubble_sort.c
	gcc $(CFLAGS) -o bubble_sort sorting/bubble_sort.c -I.

sorting: insertion_sort quick_sort bubble_sort
all: sorting

clean:
	del *.exe
	del *.o