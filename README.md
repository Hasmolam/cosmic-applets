# COSMIC Time Applet with Calendar & Agenda Integration

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Built for COSMIC](https://img.shields.io/badge/Desktop-COSMIC%20Epoch-orange.svg)](https://github.com/pop-os/cosmic-epoch)
[![Rust](https://img.shields.io/badge/Rust-2024%20Edition-red.svg)](https://www.rust-lang.org/)

An enhanced, drop-in replacement for the official Pop!_OS **`cosmic-applet-time`** featuring interactive calendar event dots, a scrollable agenda view, meeting URL launching, and seamless Google Calendar / CalDAV synchronization.

> **Why this exists:** The default COSMIC time applet does not yet display calendar events or public holidays ([Issue #871](https://github.com/pop-os/cosmic-applets/issues/871), [Issue #1331](https://github.com/pop-os/cosmic-applets/issues/1331)). While the ecosystem awaits a dedicated accounts daemon, this enhanced build delivers full calendar integration **today** without modifying system packages.

---

## 📸 Screenshots

| Event with Meeting Link | Empty State ("No scheduled events") |
| :---: | :---: |
| ![Event with Link](screenshots/01_english_event_with_link_june5.png) | ![Empty State](screenshots/02_english_empty_state_june18.png) |

| Standard Meeting View | Localization & Real CalDAV Sync |
| :---: | :---: |
| ![Standard Event](screenshots/03_english_meeting_june20.png) | ![Turkish i18n & CalDAV](screenshots/04_turkish_i18n_and_eds_real_event.png) |

---

## ✨ Features

- 📅 **Google Calendar, Nextcloud & CalDAV Sync:** Connects directly over D-Bus to Evolution Data Server (EDS). If you have accounts configured via GNOME Online Accounts or Evolution, all your calendars show up automatically.
- 🏖️ **Public Holidays & Local ICS Files:** Automatically discovers and parses standard `.ics` iCalendar files in `~/.local/share/calendars/` or via `COSMIC_CALENDAR_ICS` with **zero system daemons**.
- 🔘 **Event Indicator Dots:** High-visibility indicator dots positioned directly beneath dates on the 42-day calendar matrix for days with scheduled events.
- 📋 **Scrollable Agenda View:** Displays the selected day's events chronologically with all-day events at the top, meeting summaries, time intervals, and optional event locations.
- 🔗 **Direct Meeting Launcher:** Detected meeting URLs (Google Meet, Zoom, Microsoft Teams) display a direct launch button that opens the meeting in your default browser via secure Wayland tokens.
- ⚡ **Stale-While-Revalidate LRU Caching:** 0ms instantaneous month transitions with in-memory 6-month LRU caching and a 60-second TTL to ensure events stay up-to-date automatically.
- 🔋 **Zero-Idle IPC & Battery Protection:** Zero background polling, zero D-Bus queries, and zero CPU wakeups when the popup is closed.
- 🌐 **Full Fluent i18n Support:** Fully localized in English and Turkish, with fallback support for all COSMIC languages.
- 🔄 **Safe & Reversible:** Installed entirely in user space (`~/.local/bin/`); can be enabled or reverted to the vanilla system applet in 1 second.

---

## 🚀 Quick Install

### Method 1: One-Line Installer
Run the following in your terminal to automatically install and restart the panel:

```bash
curl -fsSL https://raw.githubusercontent.com/Hasmolam/cosmic-applets/feat/calendar-events-integration/install.sh | bash
```

### Method 2: From Source
If you prefer compiling locally using the Rust toolchain:

```bash
git clone https://github.com/Hasmolam/cosmic-applets.git
cd cosmic-applets
git checkout feat/calendar-events-integration
./install.sh
```

---

## 🗑️ Uninstall / Revert

To instantly remove the custom build and revert to the vanilla system clock:

```bash
./uninstall.sh
```
Or simply:
```bash
rm -f ~/.local/bin/cosmic-applet-time && killall cosmic-panel
```

---

## ⚙️ How It Works (Architecture)

The applet uses a modular **`CalendarBackend`** abstraction trait:

```text
┌────────────────────────────────────────────────────────┐
│                   cosmic-applet-time                   │
│   (Calendar Grid Dots + Agenda View + Wayland Popup)   │
└───────────────────────────┬────────────────────────────┘
                            │
                  ┌─────────┴─────────┐
                  ▼                   ▼
        ┌──────────────────┐ ┌──────────────────┐
        │  LocalIcsBackend │ │    EdsBackend    │
        │ (~/.local/share/ │ │  (D-Bus query to │
        │    calendars)    │ │   CalDAV / EDS)  │
        └──────────────────┘ └──────────────────┘
```

When the upcoming COSMIC Accounts daemon is released, a third `AccountsBackend` can be added to this trait in less than 50 lines of code without altering the user interface.

---

## 📝 License

Licensed under the **GNU General Public License v3.0 (GPL-3.0-only)** in accordance with System76's `cosmic-applets` project.
