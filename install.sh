#!/bin/sh
# avdoc installer
# Usage: curl -sSf https://raw.githubusercontent.com/Aryan-202/avdoc/main/install.sh | sh

set -e

REPO="Aryan-202/avdoc"
BINARY="avdoc"
INSTALL_DIR="/usr/local/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo "${BLUE}Installing avdoc...${NC}"

# Detect OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64)  ASSET="avdoc-linux-x86_64" ;;
      aarch64) ASSET="avdoc-linux-arm64" ;;
      *)       echo "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
    esac
    ;;
  Darwin)
    case "$ARCH" in
      x86_64)  ASSET="avdoc-macos-x86_64" ;;
      arm64)   ASSET="avdoc-macos-arm64" ;;
      *)       echo "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
    esac
    ;;
  *)
    echo "${RED}Unsupported OS: $OS${NC}"
    echo "Please download manually from: https://github.com/$REPO/releases"
    exit 1
    ;;
esac

# Get latest release version
LATEST=$(curl -sSf "https://api.github.com/repos/$REPO/releases/latest" \
  | grep '"tag_name"' \
  | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST" ]; then
  echo "${RED}Could not fetch latest release. Check your internet connection.${NC}"
  exit 1
fi

echo "Latest version: $LATEST"
echo "Downloading $ASSET..."

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST/$ASSET"

# Download binary
TMP=$(mktemp)
curl -sSfL "$DOWNLOAD_URL" -o "$TMP"
chmod +x "$TMP"

# Install (try /usr/local/bin, fallback to ~/.local/bin)
if [ -w "$INSTALL_DIR" ]; then
  mv "$TMP" "$INSTALL_DIR/$BINARY"
else
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
  mv "$TMP" "$INSTALL_DIR/$BINARY"
  echo "Installed to $INSTALL_DIR (no sudo access to /usr/local/bin)"
  
  # Remind user to add to PATH if needed
  case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
      echo ""
      echo "Add this to your shell profile (~/.bashrc or ~/.zshrc):"
      echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
      ;;
  esac
fi

echo "${GREEN}✅ avdoc $LATEST installed successfully!${NC}"
echo "Run 'avdoc --help' to get started."