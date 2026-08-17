#!/usr/bin/env sh
set -e

# Colors for terminal output
BOLD="\033[1m"
GREEN="\033[32m"
CYAN="\033[36m"
YELLOW="\033[33m"
RESET="\033[0m"

echo "${CYAN}${BOLD}⚡ Installing docgen-cli (Universal Document Engine & MCP Server)...${RESET}"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Darwin)
        TARGET="aarch64-apple-darwin"
        if [ "$ARCH" = "x86_64" ]; then
            TARGET="x86_64-apple-darwin"
        fi
        ;;
    Linux)
        TARGET="x86_64-unknown-linux-gnu"
        if [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
            TARGET="aarch64-unknown-linux-gnu"
        fi
        ;;
    *)
        echo "Unsupported operating system: $OS"
        echo "Please build from source using 'cargo install --git https://github.com/0xkaizoku/docgen-cli'"
        exit 1
        ;;
esac

# Determine installation directory
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ] || [ "$EUID" != "0" 2>/dev/null ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

if command -v cargo >/dev/null 2>&1; then
    echo "📦 Cargo detected. Building native binary for maximum performance..."
    cargo install --git https://github.com/0xkaizoku/docgen-cli --force
else
    echo "📥 Downloading pre-built binary for ${TARGET}..."
    DOWNLOAD_URL="https://github.com/0xkaizoku/docgen-cli/releases/latest/download/docgen-cli-${TARGET}.tar.gz"
    TEMP_DIR="$(mktemp -d)"
    
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$DOWNLOAD_URL" | tar -xz -C "$TEMP_DIR"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$DOWNLOAD_URL" | tar -xz -C "$TEMP_DIR"
    else
        echo "Neither curl nor wget was found. Please install curl or wget."
        exit 1
    fi
    
    if [ -f "$TEMP_DIR/docgen" ]; then
        mv "$TEMP_DIR/docgen" "$INSTALL_DIR/docgen"
    elif [ -f "$TEMP_DIR/docgen-cli" ]; then
        mv "$TEMP_DIR/docgen-cli" "$INSTALL_DIR/docgen"
    fi
    ln -sf "$INSTALL_DIR/docgen" "$INSTALL_DIR/doc"
    ln -sf "$INSTALL_DIR/docgen" "$INSTALL_DIR/docgen-cli"
    rm -rf "$TEMP_DIR"
    chmod +x "$INSTALL_DIR/docgen" "$INSTALL_DIR/doc" "$INSTALL_DIR/docgen-cli"
fi

echo "${GREEN}${BOLD}✅ Successfully installed docgen, doc, and docgen-cli to $INSTALL_DIR!${RESET}"

# Optional AI tool auto-configuration
if command -v docgen >/dev/null 2>&1; then
    echo "${YELLOW}🤖 Configuring local AI environments (optional)...${RESET}"
    docgen init-ai >/dev/null 2>&1 || true
fi

echo ""
echo "${BOLD}🎉 Installation Complete!${RESET}"
echo "Try running:"
echo "  ${CYAN}docgen convert input.md -o output.docx${RESET}"
echo "  ${CYAN}doc convert data.json -o report.xlsx${RESET}"
echo "  ${CYAN}docgen list-templates${RESET}"
