#!/bin/sh

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <year> <day>"
    exit 1
fi

year=$1
day=$(printf "%02d" $2)

cargo run --release ${year} ${day}
