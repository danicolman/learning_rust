#!/bin/bash

cargo new $1 && cd $1
touch .gitignore
echo "/target" > .gitignore
