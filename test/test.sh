#!/bin/bash

if [ -z $1 ]; then
    echo "no endpoint set"
    exit 1
fi

SCRIPT=$(readlink -f "$0")
WORKDIR=$(dirname "$SCRIPT")/result

echo Creating directory $WORKDIR

mkdir -p $WORKDIR

echo Running tests against $1

report="${WORKDIR}/REPORT.md"

cat > $report <<- EOM
## Test results
EOM

declare -a formats=("jpeg" "png" "webp" "gif")
declare -a widths=("200" "400")
declare -a heights=("400" "600")

for format in "${formats[@]}"
do
  cat >> $report <<- EOM
**$format**
|Resolution|Size|
|----------|----|
EOM

  for width in "${widths[@]}"
  do
    for height in "${heights[@]}"
    do
      filename=${width}x${height}.${format}

      json=$(jq -n --arg format $format --arg width $width --arg height $height '.source="https://upload.wikimedia.org/wikipedia/commons/e/e9/Hue_alpha_falloff.png" | .format="\($format)" | .width=($width | tonumber) | .height=($height | tonumber)')

      output=${WORKDIR}/${filename}

      curl -sSL "$1" -H "Content-type: application/json" -d "$json" -o "$output"

      filesize=$(stat -c "%s" "$output" | numfmt --to=iec-i)

      printf "|\`${width}\`×\`${height}\`|\`$filesize\`|\n" >> $report

      echo $output
    done
  done
done
