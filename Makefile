.PHONY: clean default

line: line.rs libfoo.a
	rustc line.rs -L.

libfoo.a: foo.o
	ar cr libfoo.a foo.o

foo.o: foo.c
	gcc foo.c -c -o foo.o


clean:
	-rm line foo.o libfoo.a
