obj:
	@gcc -c main.c -o main.o && gcc -c sub.c -o sub.o

build:
	@gcc -Wl,--strip-all,--build-id=none -nostdlib -static main.c sub.c -o main
