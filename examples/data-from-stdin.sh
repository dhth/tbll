#/usr/bin/env bash

cat ./examples/movies.txt | tbll "$@" \
    -s \
    --headers 'Title,Year,Director,Genre'
