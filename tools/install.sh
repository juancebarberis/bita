#!/bin/sh

set +e

# Set variables
url="https://github.com/juancebarberis/bita/releases/latest/download/bita"
install_path="/usr/local/bin"

# Download the binary
echo "Downloading Bita binary from $url..."
curl -L -o "$install_path/bita" "$url"

# Check if download was successful
if [ $? -ne 0 ]; then
    echo "Failed to download the Bita binary from $url."
    echo "Hint: if you got a permission denied error, try running with sudo."
    exit 1
fi

# Make the binary executable
chmod +x "$install_path/bita"

set -e

echo "Bita installation complete!"
