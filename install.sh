#!/usr/bin/env bash
# StarFetch install script (Linux / macOS / BSD)
# Usage: curl -fsSL https://raw.githubusercontent.com/Linus-Shyu/StarFetch_Core/master/install.sh | bash

set -e

REPO="Linus-Shyu/StarFetch_Core"
INSTALL_DIR=""

get_latest_version() {
  if command -v jq &>/dev/null; then
    curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | jq -r '.tag_name'
  else
    curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/'
  fi
}

detect_asset() {
  local os arch
  os=$(uname -s)
  arch=$(uname -m)
  case "$os" in
    Darwin)
      case "$arch" in
        x86_64) echo "starfetch-x86_64-apple-darwin.tar.gz" ;;
        arm64|aarch64) echo "starfetch-aarch64-apple-darwin.tar.gz" ;;
        *) echo "unsupported: $os $arch" ;;
      esac
      ;;
    Linux|*BSD*|GNU*)
      case "$arch" in
        x86_64|amd64) echo "starfetch-x86_64-unknown-linux-gnu.tar.gz" ;;
        aarch64|arm64) echo "starfetch-aarch64-unknown-linux-gnu.tar.gz" ;;
        *) echo "unsupported: $os $arch" ;;
      esac
      ;;
    *)
      echo "unsupported: $os $arch"
      ;;
  esac
}

choose_install_dir() {
  if [ -w /usr/local/bin ] 2>/dev/null; then
    INSTALL_DIR="/usr/local/bin"
  elif [ -w /usr/bin ] 2>/dev/null; then
    INSTALL_DIR="/usr/bin"
  else
    INSTALL_DIR="${HOME}/.local/bin"
    mkdir -p "$INSTALL_DIR"
    if ! echo ":$PATH:" | grep -q ":$INSTALL_DIR:"; then
      echo "Add to your shell profile (e.g. ~/.bashrc or ~/.zshrc):"
      echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi
  fi
}

main() {
  echo "StarFetch installer"
  VERSION=$(get_latest_version)
  [ -z "$VERSION" ] && { echo "Could not get latest version"; exit 1; }
  VERSION=${VERSION#v}
  ASSET=$(detect_asset)
  if [ -z "$ASSET" ] || [ "$ASSET" != "${ASSET#unsupported}" ]; then
    echo "Unsupported platform: $(uname -s) $(uname -m)"
    exit 1
  fi
  URL="https://github.com/${REPO}/releases/download/v${VERSION}/${ASSET}"
  echo "Installing starfetch v${VERSION} from ${ASSET}"
  choose_install_dir
  TMP=$(mktemp -d)
  trap "rm -rf $TMP" EXIT
  if command -v curl &>/dev/null; then
    curl -fsSL -o "$TMP/starfetch.tar.gz" "$URL"
  else
    wget -q -O "$TMP/starfetch.tar.gz" "$URL"
  fi
  tar -xzf "$TMP/starfetch.tar.gz" -C "$TMP"
  if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP/starfetch" "$INSTALL_DIR/starfetch"
    chmod +x "$INSTALL_DIR/starfetch"
  else
    sudo mv "$TMP/starfetch" "$INSTALL_DIR/starfetch"
    sudo chmod +x "$INSTALL_DIR/starfetch"
  fi
  echo "Installed to $INSTALL_DIR/starfetch"
  starfetch --version 2>/dev/null || true
}

main "$@"
