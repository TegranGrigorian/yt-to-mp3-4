#!/bin/bash
echo "Installing yt-to-mp3-4 binaries..."

# Define the installation directory in the user's home directory
INSTALL_DIR="$HOME/.local/bin"

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

# Add the installation directory to PATH if not already present
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Adding $INSTALL_DIR to PATH..."
    echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.bashrc"
    echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.profile"
    export PATH="$INSTALL_DIR:$PATH"
fi

# Verify installation
if [ -x "$INSTALL_DIR/yt-dlp" ] && [ -x "$INSTALL_DIR/ffmpeg" ] && [ -x "$INSTALL_DIR/ffprobe" ]; then
    echo "Installation complete! Binaries are now available in $INSTALL_DIR."
    echo "Please restart your terminal or run 'source ~/.bashrc' to update your PATH."
else
    echo "Installation failed. Please check the script and try again."
    exit 1
fi