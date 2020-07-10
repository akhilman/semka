#!/bin/bash
ipfs add --pin=false -r $1 | tee /dev/stderr | tail -n 1 | sed 's/^added \(\w\+\) dist$/https:\/\/ipfs.io\/ipfs\/\1/'
