#!/bin/bash
set -e

DOCKER_PASSWORD=$1
sudo apt install -y jq
./ci/ci.sh stzups/"$(./ci/cargo-metadata.sh name)":"$(./ci/cargo-metadata.sh version)" "stzups" "$DOCKER_PASSWORD"