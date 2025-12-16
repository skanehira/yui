#!/bin/bash
. "$(dirname "$0")"/shared.sh

cat <<EOF | gcc -xc -c -o "$t/main.o" -
#include <stdio.h>

extern char name[];

int main() {
  printf("Hello, %s\n", name);
}
EOF

cat <<EOF | gcc -xc -c -o "$t/sub.o" -
char name[] = "Yui";
EOF

$linker "$t/exe" "$t/main.o" "$t/sub.o"

result=$("./$t/exe")
expect="Hello, Yui"

if [ "$result" != "$expect" ]; then
  echo "Failed: expected $expect, got $result"
  exit 1
fi
