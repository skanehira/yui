bench:
	@cargo +nightly bench

build-obj:
	@cd src/parser/fixtures && gcc -c main.c -o main.o && gcc -c sub.c -o sub.o

test:
	@cargo nextest run

build: build-obj
	@cargo run -- a.out src/parser/fixtures/main.o src/parser/fixtures/sub.o

run: build
	@./a.out; echo $$?

dump: build
	@readelf -l -s -S a.out; objdump -d a.out
