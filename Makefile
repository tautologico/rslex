all: rslex

rslex: main.rs parser.rs buffer.rs regexp.rs
	rustc -o rslex main.rs

test: main.rs parser.rs buffer.rs regexp.rs 
	rustc -o test --test main.rs

clean:
	rm -f *.o
	rm -f rslex
	rm -R -f rslex.dSYM/
	rm -f test
	rm -R -f test.dSYM/
