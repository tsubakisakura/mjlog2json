#!/bin/bash -eu
# Usage:
# Create a download folder and run this script.
# For files prior to 2023, you can download the same files from the URL below.
# http://tenhou.net/sc/raw/scraw2023.zip
# http://tenhou.net/sc/raw/scraw2022.zip
# http://tenhou.net/sc/raw/scraw2021.zip
# ...

url="https://tenhou.net/sc/raw/list.cgi?old"
pairs="$(curl $url | grep -o "file:'[^']*',size:[0-9]*" | sed -E "s/file:'([^']*)',size:([0-9]*)/\1 \2/g" | grep "scc")"

echo "$pairs" | while IFS=' ' read -r file size; do
  if [ -f "$file" ]; then
    actual_size=$(stat -c "%s" "$file")
    if [ "$actual_size" -eq "$size" ]; then
      echo "Exist: $file"
      continue
    fi
  fi
  echo "Download: $file"
  mkdir -p "$(dirname $file)"
  curl -s "https://tenhou.net/sc/raw/dat/$file" > $file
done
