#!/bin/bash

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    maturin publish
elif [[ "$OSTYPE" == "msys" ]]; then
    maturin publish -i "C:\Python\Python311\python.exe"
fi
