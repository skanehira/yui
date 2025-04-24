bench:
	@cargo +nightly bench

test:
	@cargo nextest run

build:
	@cargo run -- a.out src/parser/fixtures/main.o src/parser/fixtures/sub.o

run: build
	@./a.out; echo $$?

dump: build
	@readelf -l -s -S a.out; objdump -d a.out
