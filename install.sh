#!/bin/bash

SCRIPT_DIR=$(dirname "$(realpath "$0")")
cd "$SCRIPT_DIR/src" || exit
dotnet build --configuration Release -v m
ln -s $SCRIPT_DIR/src/bin/Release/net8.0/sparv /usr/local/bin/sparv
