#!/bin/bash

if [ -z $1 ]; then
    echo "no endpoint set"
    exit 1
fi

SCRIPT=$(readlink -f "$0")
WORKDIR=$(dirname "$SCRIPT")/result

echo Creating directory $WORKDIR

mkdir -p $WORKDIR

declare -a formats=("jpeg" "png" "webp" "gif")
declare -a widths=("200" "400")
declare -a heights=("400" "600")

for format in "${formats[@]}"
do
  for width in "${widths[@]}"
  do
    for height in "${heights[@]}"
    do
      filename=${width}x${height}.${format}

      json=$(jq -n --arg format $format --arg width $width --arg height $height '.source="https://upload.wikimedia.org/wikipedia/commons/e/e9/Hue_alpha_falloff.png" | .format="\($format)" | .width=($width | tonumber) | .height=($height | tonumber)')

      output=${WORKDIR}/${filename}
      
      curl -d "$json" -H "Content-type: application/json" -o "$output" -s "$1"

      echo $filename
    done
  done
done

# echo "Testing $1"