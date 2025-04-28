CFLAGS = -O3
LDFLAGS = ""
IFLAGS = ""
GCC = gcc

ifeq ($(OS),Windows_NT)
	IFLAGS = -I${VCPKG_ROOT}/installed/x64-windows/include
	LDFLAGS = -L${VCPKG_ROOT}/installed/x64-windows/lib
endif

all:
	$(GCC) $(CFLAGS) -o main main.c -I. $(IFLAGS) $(LDFLAGS) -lcsv -DUSE_CSV

nocsv:
	$(GCC) $(CFLAGS) -o main main.c -I. $(IFLAGS) $(LDFLAGS) -lcsv

clean:
	del *.exe
	del *.o
