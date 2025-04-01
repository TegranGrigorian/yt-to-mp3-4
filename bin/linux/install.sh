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
USR_DIR ="$HOME/usr/local/bin/yt-dlp"
#fix bin issue
echo "Fixing binary issue..."
echo "If issue occurs please check the ./bin/linux folder"
echo "binaries could of not been installed correcetly!"
sudo mkdir -p ./bin/linux 


echo "Binaries in bin linux folder!"

#give them defaults
INSTALL_DIR="$HOME/Downloads/yt-to-mp3-4-linux/linux/bin/linux"

#copy bin
sudo cp ffmpeg "$INSTALL_DIR/"
sudo cp ffprobe "$INSTALL_DIR/"
sudo cp yt-dlp "$INSTALL_DIR/"

#perms
sudo chmod +x "$INSTALL_DIR/yt-dlp"
sudo chmod +x "$INSTALL_DIR/ffmpeg"
sudo chmod +x "$INSTALL_DIR/ffprobe"

#echos
echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> ~/.bashrc
echo "yt-dlp installed to $INSTALL_DIR and added to PATH."
echo "Installation complete! Binaries are now available in /usr/local/bin."
