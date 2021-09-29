#!/bin/bash

echo
echo "this is test.sh, and here are some of the arguments:"
echo "$0 $1 $2 $3 $4 $5 $6 $7"
#env
[ ! -t 0 ] && echo "piped output: " && cat /dev/stdin
echo
exit 1