#!/bin/bash

# Find all .rs files
find . -name "*.rs" |
while read -r file
do
    # Extract struct names from each file, excluding lines starting with //
    grep -v '^//' "$file" | grep -oP 'struct \K\w+'
done | sort | uniq -d

# If the above command outputs anything, then there are duplicate struct names