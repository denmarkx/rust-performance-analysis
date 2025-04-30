CFLAGS = -O3
LDFLAGS = ""
IFLAGS = ""
GCC = gcc

ifeq ($(OS),Windows_NT)
	IFLAGS = ${VCPKG_ROOT}/installed/x64-windows/include
	LDFLAGS = ${VCPKG_ROOT}/installed/x64-windows/lib
else
	OS_NAME := $(shell uname -s)
        ifeq ($(OS_NAME),Darwin)
		IFLAGS = /opt/homebrew/include
		LDFLAGS = /opt/homebrew/lib
	endif
endif

all:
	$(GCC) $(CFLAGS) -o main main.c -I. -I$(IFLAGS) -L$(LDFLAGS) -lcsv -DUSE_CSV

nocsv:
	$(GCC) $(CFLAGS) -o main main.c -I. -I$(IFLAGS) -L$(LDFLAGS) -lcsv

clean:
	del *.exe
	del *.o
