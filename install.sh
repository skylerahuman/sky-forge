#!/usr/bin/env sh
set -eu

REPO="skylerahuman/sky-forge"
BIN_NAME="sky"
INSTALL_DIR="${FORGE_INSTALL_DIR:-$HOME/.local/bin}"

# Flags
VERSION=""
MUSL_VARIANT="true"

usage() {
    cat <<EOF
Usage: install.sh [OPTIONS]

Install SkyBolt from GitHub releases at https://github.com/skylerahuman/sky-forge.
The installed binary is named 'sky'.

Options:
  --version <tag>   Install a specific release (default: latest)
  --gnu             Use the GNU libc variant on Linux (default: musl/static)
  -h, --help        Show this help message
EOF
}

while [ $# -gt 0 ]; do
    case "$1" in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --gnu)
            MUSL_VARIANT="false"
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1" >&2
            usage >&2
            exit 1
            ;;
    esac
done

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux)   OS_TAG="linux" ;;
    Darwin)  OS_TAG="darwin" ;;
    *)
        echo "Unsupported OS: $OS" >&2
        exit 1
        ;;
esac

# Detect architecture
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64)          ARCH_TAG="x86_64" ;;
    aarch64|arm64)   ARCH_TAG="aarch64" ;;
    *)
        echo "Unsupported architecture: $ARCH" >&2
        exit 1
        ;;
esac

# Build target triple
if [ "$OS_TAG" = "linux" ]; then
    if [ "$MUSL_VARIANT" = "true" ]; then
        TARGET="${ARCH_TAG}-unknown-linux-musl"
    else
        TARGET="${ARCH_TAG}-unknown-linux-gnu"
    fi
elif [ "$OS_TAG" = "darwin" ]; then
    TARGET="${ARCH_TAG}-apple-darwin"
fi

BINARY_FILENAME="${BIN_NAME}-${TARGET}"

# Resolve version tag
if [ -z "$VERSION" ]; then
    echo "Fetching latest release..."
    VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' \
        | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')"
    if [ -z "$VERSION" ]; then
        echo "Failed to determine latest release. Specify --version manually." >&2
        exit 1
    fi
fi

DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY_FILENAME}"

echo "Installing SkyBolt ${VERSION} as ${BIN_NAME} (${TARGET})..."

# Download to temp file
TMP_FILE="$(mktemp)"
trap 'rm -f "$TMP_FILE"' EXIT

if ! curl -fSL "$DOWNLOAD_URL" -o "$TMP_FILE"; then
    echo "Download failed: $DOWNLOAD_URL" >&2
    exit 1
fi

# Verify non-empty
if [ ! -s "$TMP_FILE" ]; then
    echo "Downloaded file is empty." >&2
    exit 1
fi

# Install
mkdir -p "$INSTALL_DIR"
DEST="${INSTALL_DIR}/${BIN_NAME}"
mv "$TMP_FILE" "$DEST"
chmod +x "$DEST"

echo "SkyBolt installed to ${DEST}"
echo "Run 'sky setup' to install the ZSH plugin."

# Warn if INSTALL_DIR is not in PATH
case ":${PATH}:" in
    *":${INSTALL_DIR}:"*)
        ;;
    *)
        echo ""
        echo "WARNING: ${INSTALL_DIR} is not in your PATH."
        echo "Add the following to your ~/.zshrc or ~/.bashrc:"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
        ;;
esac
