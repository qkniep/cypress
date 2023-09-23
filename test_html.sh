#!/bin/bash

for i in $(seq 1 10)
do
  http "https://en.wikipedia.org/wiki/$i"
  sleep 1
done
