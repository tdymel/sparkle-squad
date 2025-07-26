#!/bin/bash

# Set the quality level for conversion
QUALITY=60

# Function to convert images
convert_images() {
  local extension="$1"
  local find_command="find . -type f -iname '*.${extension}'"

  eval "$find_command" | while read -r file; do
    # Construct the output file name by replacing the extension with .webp
    output="${file%.$extension}.webp"

    # Check if the output file already exists
    if [[ ! -f "$output" ]]; then
      # Convert the image to WebP using cwebp
      cwebp -q $QUALITY "$file" -o "$output"

      # Optionally, print a message for each converted file
      echo "Converted: $file to $output"
    else
      echo "Skipped (already converted): $file"
    fi
  done
}

# Convert PNG files
convert_images "png"

# Convert JPG files
convert_images "jpg"