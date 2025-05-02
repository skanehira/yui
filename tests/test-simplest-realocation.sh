#!/bin/bash
. "$(dirname "$0")"/shared.sh

cat <<EOF | gcc -xc -c -o "$t/main.o" -
__asm__(
      ".global _start\n"
      "_start:\n"
      "    adr     x0, x\n"
      "    ldr     w0, [x0]\n"
      "    mov     x8, #93\n"
      "    svc     #0\n"
);
EOF

cat <<EOF | gcc -xc -c -o "$t/sub.o" -
int x = 11;
EOF

$linker "$t/exe" "$t/main.o" "$t/sub.o"

"./$t/exe"

result=$?
expect=11

if [ $result -ne $expect ]; then
  echo "Failed: expected $expect, got $result"
  exit 1
fi
