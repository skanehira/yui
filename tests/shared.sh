export LC_ALL=C
TESTDIR=out

testname=$(basename "$0" .sh)
t=$TESTDIR/$testname
mkdir -p "$t"

echo -n "Testing $testname ... "
set -o pipefail
set -x
