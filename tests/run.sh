#!/bin/bash

# This script is used to run the tests for the Yui compiler.
for test in test-*.sh; do
  bash "$test"
done
