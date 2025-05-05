#!/bin/bash

# This script is used to run the tests for the Yui compiler.
for test in test-*.sh; do
  exit=$(bash "$test")
  if [ $? -ne 0 ]; then
    echo "Test $test failed with exit code $exit"
    exit 1
  fi
done
