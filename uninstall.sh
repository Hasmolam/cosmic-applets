#!/usr/bin/env bash
# ==============================================================================
# COSMIC Time Applet with Calendar Integration - Uninstaller
# Author: Hasan Hüseyin Yolcu <hasanhuseyinyolcu25@gmail.com>
# License: GPL-3.0-only
# ==============================================================================

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

TARGET_BIN="${HOME}/.local/bin/cosmic-applet-time"

echo -e "${BLUE}${BOLD}=== COSMIC Time Applet with Calendar - Uninstaller ===${NC}\n"

if [[ -f "${TARGET_BIN}" || -L "${TARGET_BIN}" ]]; then
    echo -e "${BLUE}[*] Removing custom applet binary from ${TARGET_BIN}...${NC}"
    rm -f "${TARGET_BIN}"
    echo -e "${BLUE}[*] Restarting cosmic-panel to restore default system time applet...${NC}"
    killall cosmic-panel 2>/dev/null || true
    echo -e "${GREEN}${BOLD}✓ Success!${NC} Reverted to default system time applet."
else
    echo -e "${GREEN}No custom applet found at ${TARGET_BIN}. System is already using default applet.${NC}"
fi
