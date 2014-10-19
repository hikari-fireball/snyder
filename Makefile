CC=clang++
CFLAGS=-c -Wall -Wextra -std=c++11 -g
INCLUDE=-I $(INCDIR)
LIBS=

SRCDIR=src
OBJDIR=obj
BINDIR=bin
INCDIR=src/include

.PHONY: all clean

default: all

all: $(BINDIR)/sudoku

$(BINDIR)/sudoku: $(OBJDIR)/sudoku.o
	$(CC) $^ -o $@ $(LIBS)

$(OBJDIR)/sudoku.o: $(SRCDIR)/sudoku.cpp
	$(CC) $(CFLAGS) $(INCLUDE) $^ -o $@

clean:
	rm -f $(BINDIR)/* $(OBJDIR)/*
