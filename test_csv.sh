#!/bin/bash

echo "name,id"

for i in $(seq 1 100)
do
  echo "\"test $i\",$i"
  sleep 0.1
done
