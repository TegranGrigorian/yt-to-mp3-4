#!/bin/bash
echo "Installing yt-to-mp3-4 binaries..."

# Create directories
sudo mkdir -p /usr/local/bin

# Copy binaries to /usr/local/bin
echo "Copying binaries to /usr/local/bin..."
sudo cp ffmpeg /usr/local/bin/
sudo cp ffprobe /usr/local/bin/
sudo cp yt-dlp /usr/local/bin/

# Set executable permissions
echo "Setting executable permissions..."
sudo chmod +x /usr/local/bin/ffmpeg
sudo chmod +x /usr/local/bin/ffprobe
sudo chmod +x /usr/local/bin/yt-dlp

echo "Installation complete! Binaries are now available in /usr/local/bin."
