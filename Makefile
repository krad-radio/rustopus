all:
	rustc --lib opus.rs
	rustc opus_test.rs -L .

check: all
	./opus_test

clean:
	rm *.so
