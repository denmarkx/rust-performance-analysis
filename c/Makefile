quick_sort: sorting/quick_sort.c
	gcc -o quick_sort sorting/quick_sort.c -I.

insertion_sort: sorting/insertion_sort.c
	gcc -o insertion_sort sorting/insertion_sort.c -I.

sorting: insertion_sort quick_sort
all: sorting

clean:
	del *.exe
	del *.o