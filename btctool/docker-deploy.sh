#!/usr/bin/env bash

set -ex

TIMESTAMP=$(date +%s)
DOCKER_TAG=igorsyl/btctool:$TIMESTAMP

docker build -t $DOCKER_TAG . && docker push $DOCKER_TAG
echo $DOCKER_TAG