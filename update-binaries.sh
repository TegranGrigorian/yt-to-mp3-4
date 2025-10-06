#!/bin/bash

# Script to update ffmpeg and yt-dlp binaries to the latest versions
# Usage: ./update-binaries.sh

set -e  # Exit on any error

echo "Updating ffmpeg and yt-dlp to latest versions..."
echo "=================================================="

# Create directories if they don't exist
mkdir -p bin/linux
mkdir -p bin/windows

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to download with progress bar
download_with_progress() {
    local url="$1"
    local output="$2"
    local description="$3"
    
    echo "Downloading $description..."
    if command_exists wget; then
        wget --progress=bar:force -O "$output" "$url"
    elif command_exists curl; then
        curl -L --progress-bar -o "$output" "$url"
    else
        echo "Error: Neither wget nor curl is available"
        exit 1
    fi
}

# Update Linux binaries
echo ""
echo "Updating Linux binaries..."
echo "------------------------------"

# Download latest ffmpeg for Linux
echo "Updating ffmpeg..."
download_with_progress \
    "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz" \
    "ffmpeg-linux.tar.xz" \
    "ffmpeg (Linux)"

echo "Extracting ffmpeg..."
tar -xf ffmpeg-linux.tar.xz --strip-components=1 -C bin/linux
chmod +x bin/linux/ffmpeg bin/linux/ffprobe

# Clean up unnecessary files
echo "Cleaning up unnecessary files..."
rm -rf bin/linux/model
rm -rf bin/linux/manpages
rm -f bin/linux/GPLv3.txt
rm -f bin/linux/readme.txt
rm -f bin/linux/install.sh
rm -f bin/linux/qt-faststart
rm -f ffmpeg-linux.tar.xz

# Download latest yt-dlp for Linux
echo "Updating yt-dlp..."
download_with_progress \
    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp" \
    "bin/linux/yt-dlp" \
    "yt-dlp (Linux)"

chmod +x bin/linux/yt-dlp

# Update Windows binaries
echo ""
echo "Updating Windows binaries..."
echo "-------------------------------"

# Download latest yt-dlp for Windows
echo "Updating yt-dlp.exe..."
download_with_progress \
    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe" \
    "bin/windows/yt-dlp.exe" \
    "yt-dlp (Windows)"

# Download latest ffmpeg for Windows
echo "Updating ffmpeg.exe..."
# Note: This downloads a zip file with ffmpeg binaries
download_with_progress \
    "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip" \
    "ffmpeg-windows.zip" \
    "ffmpeg (Windows)"

echo "Extracting ffmpeg.exe..."
if command_exists unzip; then
    unzip -j ffmpeg-windows.zip "*/bin/ffmpeg.exe" -d bin/windows/
    unzip -j ffmpeg-windows.zip "*/bin/ffprobe.exe" -d bin/windows/
    rm ffmpeg-windows.zip
else
    echo "Warning: unzip not found. Please manually extract ffmpeg.exe and ffprobe.exe from ffmpeg-windows.zip to bin/windows/"
fi

# Display versions
echo ""
echo "Update complete! Current versions:"
echo "====================================="

echo "Linux versions:"
if [ -f "bin/linux/ffmpeg" ]; then
    echo "   ffmpeg: $(bin/linux/ffmpeg -version 2>/dev/null | head -n1 | cut -d' ' -f3)"
else
    echo "   ffmpeg: Not found"
fi

if [ -f "bin/linux/yt-dlp" ]; then
    echo "   yt-dlp: $(bin/linux/yt-dlp --version 2>/dev/null)"
else
    echo "   yt-dlp: Not found"
fi

echo ""
echo "Windows versions:"
if [ -f "bin/windows/yt-dlp.exe" ]; then
    echo "   yt-dlp.exe: Available (cannot check version on Linux)"
else
    echo "   yt-dlp.exe: Not found"
fi

if [ -f "bin/windows/ffmpeg.exe" ]; then
    echo "   ffmpeg.exe: Available (cannot check version on Linux)"
else
    echo "   ffmpeg.exe: Not found"
fi

echo ""
echo "All binaries updated successfully!"
echo "Linux binaries are in: bin/linux/"
echo "Windows binaries are in: bin/windows/"