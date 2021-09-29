#!/bin/bash

echo "piped input" | dockron -v -i -n 5 -d 100 ./test_script.sh hello hi > output