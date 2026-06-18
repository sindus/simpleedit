#!/usr/bin/env bash
set -euo pipefail

REPO="sindus/tincta-V2"

# ── helpers ──────────────────────────────────────────────────────────────────
info()  { echo "  $*"; }
ok()    { echo "✓ $*"; }
err()   { echo "✗ $*" >&2; exit 1; }

need() {
    command -v "$1" &>/dev/null || err "Required tool not found: $1 — please install it and retry."
}

# ── resolve latest release ───────────────────────────────────────────────────
need curl

info "Fetching latest release…"
VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' | head -1 \
    | sed 's/.*"tag_name": *"\(.*\)".*/\1/')

[ -n "$VERSION" ] || err "Could not determine latest version. Check your internet connection."
info "Latest version: ${VERSION}"

# ── platform detection ───────────────────────────────────────────────────────
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64) ;;
            *) err "Unsupported architecture: $ARCH. Only x86_64 is supported on Linux." ;;
        esac

        # Prefer .deb on Debian/Ubuntu systems
        if command -v dpkg &>/dev/null; then
            # Derive exact .deb filename from the release assets
            DEB=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
                | grep '"name"' | grep '\.deb' | head -1 \
                | sed 's/.*"name": *"\(.*\)".*/\1/')
            [ -n "$DEB" ] || err "Could not find .deb asset in latest release."

            URL="https://github.com/${REPO}/releases/download/${VERSION}/${DEB}"
            TMP=$(mktemp /tmp/tincta-XXXXXX.deb)
            info "Downloading ${DEB}…"
            curl -fsSL --progress-bar "$URL" -o "$TMP"
            info "Installing (requires sudo)…"
            sudo dpkg -i "$TMP"
            rm -f "$TMP"
        else
            # Generic Linux: extract binary to /usr/local/bin
            TARBALL="tincta-${VERSION}-x86_64-linux.tar.gz"
            URL="https://github.com/${REPO}/releases/download/${VERSION}/${TARBALL}"
            TMP=$(mktemp -d)
            info "Downloading ${TARBALL}…"
            curl -fsSL --progress-bar "$URL" | tar -xz -C "$TMP"
            info "Installing binary to /usr/local/bin (requires sudo)…"
            sudo mv "$TMP/tincta" /usr/local/bin/tincta
            sudo chmod +x /usr/local/bin/tincta
            rm -rf "$TMP"
        fi
        ;;

    Darwin)
        case "$ARCH" in
            arm64) ;;
            *) err "Only Apple Silicon (M-series) is supported on macOS." ;;
        esac

        if command -v brew &>/dev/null; then
            info "Installing via Homebrew…"
            brew tap sindus/tincta 2>/dev/null || true
            brew install tincta-v2
        else
            # Manual install: extract binary to /usr/local/bin
            TARBALL="tincta-${VERSION}-aarch64-apple-darwin.tar.gz"
            URL="https://github.com/${REPO}/releases/download/${VERSION}/${TARBALL}"
            TMP=$(mktemp -d)
            info "Downloading ${TARBALL}…"
            curl -fsSL --progress-bar "$URL" | tar -xz -C "$TMP"
            info "Installing binary to /usr/local/bin (requires sudo)…"
            sudo mv "$TMP/tincta" /usr/local/bin/tincta
            sudo chmod +x /usr/local/bin/tincta
            rm -rf "$TMP"
        fi
        ;;

    *)
        err "Unsupported OS: $OS"
        ;;
esac

# ── done ─────────────────────────────────────────────────────────────────────
ok "Tincta ${VERSION} installed successfully."
echo ""
echo "  Run:        tincta"
echo "  Open file:  tincta path/to/file"
echo ""
echo "  Uninstall (Ubuntu/Debian):  sudo apt remove tincta"
echo "  Uninstall (macOS Homebrew): brew uninstall tincta-v2"
echo "  Uninstall (manual):         sudo rm /usr/local/bin/tincta"
