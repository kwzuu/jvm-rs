#!/bin/bash
javac $(find . -name '*.java') -d class -source 1.7 -target 1.7
