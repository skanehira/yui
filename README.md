# Yui
This is a toy linker written in Rust.  
"Yui" is 結(ゆい) in Japanese and means "to bind" or "to connect."

## Example

sub.c
```c
int x = 11;
```

main.c
```c
__asm__(
      ".global _start\n"
      "_start:\n"
      "    adr     x0, x\n"
      "    ldr     w0, [x0]\n"
      "    mov     x8, #93\n"
      "    svc     #0\n"
);
```

```sh
$ gcc -c sub.c -o sub.o
$ gcc -c main.c -o main.o
$ cargo run -- a.out sub.o main.o
$ ./a.out
$ echo $?
11
```

> [!IMPORTANT]
> Please note that this repository was created for learning purposes and is not of production quality.
