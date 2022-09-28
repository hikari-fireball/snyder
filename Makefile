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

all: $(BINDIR)/snyder

$(OBJDIR):
	mkdir $@

$(BINDIR):
	mkdir $@

$(OBJDIR)/snyder.o: $(SRCDIR)/snyder.cpp $(OBJDIR)
	$(CC) $(CFLAGS) $(INCLUDE) $< -o $@

$(BINDIR)/snyder: $(OBJDIR)/snyder.o $(BINDIR)
	$(CC) $< -o $@ $(LIBS)

clean:
	rm -r $(OBJDIR) $(BINDIR)
