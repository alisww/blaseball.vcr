#!/usr/bin/zsh

for file in $1/*.zst; do
    echo "processing ${file:t:r}"
    zstdcat $file | pv | genson -d newline > $2/${file:t:r}.schema
done