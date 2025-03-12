#!/bin/bash -eu
# Usage:
#
# 1. Open this site in your browser.
# https://tenhou.net/sc/raw/
#
# 2. Download the following files from the site that opens:
# https://tenhou.net/sc/raw/scraw2023.zip
# https://tenhou.net/sc/raw/scraw2022.zip
# https://tenhou.net/sc/raw/scraw2021.zip
# https://tenhou.net/sc/raw/scraw2020.zip
# ...
#
# 3. After unzip file, run this script in the extracted folder.
#    Then you will get xml files!
#
# Note:
#
# This script can be interrupted and resumed.
# However, if you interrupt it, the file size will often become 0.
# You can check whether there are any files with size 0 by using the following command.
#
#   find . -type f -size 0

mkdir -p downloads

for gz_file in scc*.html.gz; do
  echo "Processing: ${gz_file}"

  # .*log=    -> ignore  for until log=
  # ([^"]+)   -> capture for until "
  # .*        -> ignore  for until tail
  ids="$(gunzip -dc ${gz_file} | grep "四鳳南喰赤－" | sed -E 's/.*log=([^"]+).*/\1/')"

  for id in ${ids}; do
    filename="./downloads/${id}.xml"

    if [ -e ${filename} ]; then
      if [  -s ${filename} ]; then
        echo "Exist: ${filename}"
        continue
      else
        echo "Exist: ${filename} but file size is 0, retrying download"
      fi
    fi

    echo "Download: ${filename}"
    curl -s http://tenhou.net/0/log/?${id} > $filename
  done
done
