#/usr/bin/env bash

cat ./examples/movies.txt | tbll \
    -s \
    -d '|' \
    --headers 'Title,Year,Director,Genre'
