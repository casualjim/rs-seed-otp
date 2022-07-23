#!/bin/bash

# Used in Docker build to set platform dependent variables
set -efux -o pipefail
case "${TARGETARCH}" in
    "amd64")
	echo "x86_64-unknown-linux-gnu" > /.platform
	echo "gcc-x86-64-linux-gnu" > /.compiler 
	;;
    "arm64") 
	echo "aarch64-unknown-linux-gnu" > /.platform
	echo "gcc-aarch64-linux-gnu" > /.compiler
	;;
    "arm")
	echo "armv7-unknown-linux-gnueabihf" > /.platform
	echo "gcc-arm-linux-gnueabihf" > /.compiler
	;;
    "ppc64le")
  echo "powerpc64le-unknown-linux-gnu" > /.platform
  echo "gcc-powerpc64le-linux-gnu" > /.compiler
esac