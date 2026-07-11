#!/bin/sh
set -e

# 1. Configuration
OWNER="ukangaekom"
REPO="hisho_2.0"
BINARY_NAME="hisho"

echo "Checking system compatibility for $BINARY_NAME..."

# 2. Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
if [ "$OS" = "darwin" ]; then
    OS="darwin"
elif [ "$OS" = "linux" ]; then
    OS="linux"
else
    echo "Unsupported Operating System: $OS"
    exit 1
fi

# 3. Detect Architecture
ARCH="$(uname -m)"
if [ "$ARCH" = "x86_64" ]; then
    ARCH="amd64"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    ARCH="arm64"
else
    echo "Unsupported Architecture: $ARCH"
    exit 1
fi

# 4. Construct asset name matching your workflow output
ASSET_NAME="${BINARY_NAME}-${OS}-${ARCH}"
URL="https://github.com/${OWNER}/${REPO}/releases/latest/download/${ASSET_NAME}"

# 5. Set up installation directory
DEST_DIR="$HOME/.hisho/bin"
mkdir -p "$DEST_DIR"

echo "Downloading $BINARY_NAME from GitHub Releases..."
# Download asset, but save it cleanly as exactly "hisho"
curl -sSL "$URL" -o "$DEST_DIR/$BINARY_NAME"
chmod +x "$DEST_DIR/$BINARY_NAME"

echo ""
echo "============================================="
echo "Successfully installed $BINARY_NAME!"
echo "============================================="
echo ""
echo "To use it, add the binary to your environment PATH:"
echo "export PATH=\"\$PATH:$DEST_DIR\""
echo ""
echo "Run 'source ~/.bashrc' or 'source ~/.zshrc' to apply changes."