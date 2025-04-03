#!/bin/bash
echo "Installing yt-to-mp3-4 binaries..."

# Ensure the script is run with sufficient privileges
# if [ "$EUID" -ne 0 ]; then
#     echo "Please run this script as root (e.g., using sudo)."
#     exit 1
# fi

# Define the installation directory
INSTALL_DIR="./bin/linux"

# Create the installation directory if it doesn't exist
echo "Creating installation directory at $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"

# Copy binaries to the installation directory
echo "Copying binaries to $INSTALL_DIR..."
cp ffmpeg "$INSTALL_DIR/"
cp ffprobe "$INSTALL_DIR/"
cp yt-dlp "$INSTALL_DIR/"

# Set executable permissions
echo "Setting executable permissions..."
chmod +x "$INSTALL_DIR/ffmpeg"
chmod +x "$INSTALL_DIR/ffprobe"
chmod +x "$INSTALL_DIR/yt-dlp"

# Verify installation
if [ -x "$INSTALL_DIR/yt-dlp" ] && [ -x "$INSTALL_DIR/ffmpeg" ] && [ -x "$INSTALL_DIR/ffprobe" ]; then
    echo "Installation complete! Binaries are now available in $INSTALL_DIR."
else
    echo "Installation failed. Please check the script and try again."
    exit 1
fi