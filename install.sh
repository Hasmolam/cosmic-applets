#!/usr/bin/env bash
# ==============================================================================
# COSMIC Time Applet with Calendar Integration - Installer
# Author: Hasan Hüseyin Yolcu <hasanhuseyinyolcu25@gmail.com>
# License: GPL-3.0-only
# ==============================================================================

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

TARGET_DIR="${HOME}/.local/bin"
TARGET_BIN="${TARGET_DIR}/cosmic-applet-time"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RELEASE_BIN="${REPO_ROOT}/target/release/cosmic-applet-time"
GITHUB_REPO="Hasmolam/cosmic-applets"

echo -e "${BLUE}${BOLD}=== COSMIC Time Applet with Calendar - Installer ===${NC}\n"

# 1. Ensure target directory exists and is in PATH
mkdir -p "${TARGET_DIR}"

if [[ ":${PATH}:" != *":${TARGET_DIR}:"* ]]; then
    echo -e "${YELLOW}[!] Warning:${NC} ${TARGET_DIR} is not in your PATH."
    echo -e "    Add 'export PATH=\"\$HOME/.local/bin:\$PATH\"' to your ~/.bashrc or ~/.zshrc."
fi

# 2. Determine installation source
INSTALLED=0

# Option A: Local compiled release binary in repo
if [[ -f "${RELEASE_BIN}" && -x "${RELEASE_BIN}" ]]; then
    echo -e "${BLUE}[*] Found locally compiled release binary.${NC}"
    echo -e "    Installing from: ${RELEASE_BIN}"
    rm -f "${TARGET_BIN}"
    cp "${RELEASE_BIN}" "${TARGET_BIN}"
    chmod +x "${TARGET_BIN}"
    INSTALLED=1

# Option B: Build from source if cargo is available
elif command -v cargo &>/dev/null && [[ -f "${REPO_ROOT}/Cargo.toml" ]]; then
    echo -e "${BLUE}[*] Cargo detected. Building cosmic-applet-time with --release...${NC}"
    (cd "${REPO_ROOT}" && cargo build --release -p cosmic-applet-time)
    rm -f "${TARGET_BIN}"
    cp "${RELEASE_BIN}" "${TARGET_BIN}"
    chmod +x "${TARGET_BIN}"
    INSTALLED=1

# Option C: Download latest pre-built binary from GitHub Releases
else
    echo -e "${BLUE}[*] Downloading latest pre-built binary from GitHub Releases...${NC}"
    DOWNLOAD_URL="https://github.com/${GITHUB_REPO}/releases/latest/download/cosmic-applet-time"
    if command -v curl &>/dev/null; then
        curl -fsSL "${DOWNLOAD_URL}" -o "${TARGET_BIN}" || {
            echo -e "${RED}[x] Failed to download prebuilt binary.${NC}"
            echo -e "    Please compile from source: cargo build --release -p cosmic-applet-time"
            exit 1
        }
    elif command -v wget &>/dev/null; then
        wget -qO "${TARGET_BIN}" "${DOWNLOAD_URL}" || {
            echo -e "${RED}[x] Failed to download prebuilt binary.${NC}"
            exit 1
        }
    else
        echo -e "${RED}[x] Neither curl nor wget found. Cannot download binary.${NC}"
        exit 1
    fi
    chmod +x "${TARGET_BIN}"
    INSTALLED=1
fi

if [[ "${INSTALLED}" -eq 1 ]]; then
    echo -e "\n${BLUE}[*] Restarting cosmic-panel to activate calendar applet...${NC}"
    killall cosmic-panel 2>/dev/null || true
    echo -e "${GREEN}${BOLD}✓ Success!${NC} COSMIC Time Applet with Calendar is now active."
    echo -e "  Click the date & time in your top bar to see your events and agenda."
    echo -e "  To revert anytime, run: ./uninstall.sh\n"
fi
