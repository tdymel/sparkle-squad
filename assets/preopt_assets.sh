#!/usr/bin/env bash
set -euo pipefail

DIR="${1:-.}"
MAX_WIDTH="${2:-1920}"
MAX_HEIGHT="${3:-1080}"

find "$DIR" -type f \( -iname "*.png" -o -iname "*.jpg" \) -print0 |
while IFS= read -r -d '' file; do
  lower="${file,,}"

  case "$lower" in
    *.png)
      magick "$file" -resize "${MAX_WIDTH}x${MAX_HEIGHT}>" "$file"
      pngquant --force --output "$file" -- "$file"
      ;;
    *.jpg)
      tmp="${file}.tmp.jpg"
      magick "$file" -resize "${MAX_WIDTH}x${MAX_HEIGHT}>" "$tmp"
      jpegoptim --stdout --strip-all --all-progressive -- "$tmp" > "${tmp}.opt"
      mv "${tmp}.opt" "$file"
      rm -f "$tmp"
      ;;
  esac
done
