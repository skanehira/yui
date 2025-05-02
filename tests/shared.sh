export LC_ALL=C
TESTDIR=out

testname=$(basename "$0" .sh)
t=$TESTDIR/$testname
mkdir -p "$t"

# shellcheck disable=SC2034
linker=../target/release/yui

echo -n "Testing $testname ... "
set -o pipefail
set -x
