#!/bin/sh

# Set variables
url="https://github.com/juancebarberis/bita/releases/latest/download/bita"
install_path="/usr/local/bin/"

# Create a temporary directory
temp_dir=$(mktemp -d)

# Download the binary
echo "Downloading Bita binary from $url..."
curl -L -o "$temp_dir/bita" "$url"

# Check if download was successful
if [ $? -ne 0 ]; then
    echo "Failed to download the Bita binary from $url."
    rm -rf "$temp_dir"
    exit 1
fi

# Make the binary executable
chmod +x "$temp_dir/bita"

# Install the binary to the specified path
echo "Installing Bita to $install_path..."
mv "$temp_dir/bita" "$install_path"

# Clean up temporary directory
rm -rf "$temp_dir"

echo "Bita installation complete!"
