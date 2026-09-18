#!/bin/sh

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <year> <day>"
    exit 1
fi


cargo run --release -- "$1" "$2"
