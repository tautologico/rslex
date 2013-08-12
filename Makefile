all: rslex

rslex: main.rs parser.rs
	rustc -o rslex main.rs

test: main.rs parser.rs
	rustc -o rslex_tests --test main.rs

clean:
	rm -f *.o
	rm -f rslex
	rm -R -f rslex.dSYM/
	rm -R rslex_tests
	rm -R -f rslex_tests.dSYM/
