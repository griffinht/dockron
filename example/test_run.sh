#!/bin/bash

echo "piped input" | dockron ./test_script.sh -v -i -n 5 -d 100 hello hi > output