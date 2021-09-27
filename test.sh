#!/bin/bash

echo "test.sh is running"
echo "$0 $1 $2 $3 $4 $5 $6 $7"
#env
[ ! -t 0 ] && echo "piped output: " && cat /dev/stdin
exit 1