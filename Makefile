.PHONY: clean default

line: line.rs libunbuffered.a
	rustc line.rs -L.

libunbuffered.a: unbuffered.o
	ar cr $@ $<

unbuffered.o: unbuffered.c

clean:
	-rm line libunbuffered.a unbuffered.o
