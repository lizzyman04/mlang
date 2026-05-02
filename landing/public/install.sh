#!/bin/sh
set -e

REPO="lizzyman04/mlang"

VERSION=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep -o '"tag_name": "[^"]*"' | cut -d'"' -f4)

if [ -z "$VERSION" ]; then
    echo "error: failed to fetch latest version from GitHub" >&2
    exit 1
fi

OS=$(uname -s | tr '[:upper:]' '[:lower:]')

case "$OS" in
    linux)
        FILENAME="mlang"
        INSTALL_DIR="/usr/local/bin"
        SUDO="sudo"
        ;;
    darwin)
        FILENAME="mlang"
        INSTALL_DIR="/usr/local/bin"
        SUDO="sudo"
        ;;
    msys*|mingw*|cygwin*)
        FILENAME="mlang.exe"
        INSTALL_DIR="$HOME/bin"
        SUDO=""
        ;;
    *)
        echo "error: unsupported OS: $OS" >&2
        exit 1
        ;;
esac

URL="https://github.com/$REPO/releases/download/$VERSION/$FILENAME"

echo "Downloading MLang $VERSION..."
$SUDO mkdir -p "$INSTALL_DIR"

if ! $SUDO curl -fsSL "$URL" -o "$INSTALL_DIR/$FILENAME"; then
    echo "error: download failed from $URL" >&2
    exit 1
fi

$SUDO chmod +x "$INSTALL_DIR/$FILENAME"

echo "MLang $VERSION installed to $INSTALL_DIR/$FILENAME"

if ! echo ":$PATH:" | grep -q ":$INSTALL_DIR:"; then
    echo "note: add $INSTALL_DIR to your PATH to use mlang from anywhere"
fi
