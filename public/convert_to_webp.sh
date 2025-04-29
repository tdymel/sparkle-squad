#!/bin/bash

# Set the quality level for conversion
QUALITY=60

# Find all PNG files in the current directory and subdirectories
find . -type f -iname '*.png' | while read -r file; do
  # Construct the output file name by replacing the .png extension with .webp
  output="${file%.png}.webp"
  
  # Convert the PNG to WebP using cwebp
  cwebp -q $QUALITY "$file" -o "$output"
  
  # Optionally, print a message for each converted file
  echo "Converted: $file to $output"
done

find . -type f -iname '*.jpg' | while read -r file; do
  # Construct the output file name by replacing the .png extension with .webp
  output="${file%.jpg}.webp"
  
  # Convert the PNG to WebP using cwebp
  cwebp -q $QUALITY "$file" -o "$output"
  
  # Optionally, print a message for each converted file
  echo "Converted: $file to $output"
done
