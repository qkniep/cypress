#!/bin/bash

for i in $(seq 1 100)
do
  echo "{ \"id\": $i, \"name\": \"test $i\" }"
  sleep 0.1
done
