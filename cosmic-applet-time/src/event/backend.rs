// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use jiff::civil::Date;

use crate::event::{CalendarError, CalendarEvent, ical::parse_ical_content};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Pluggable trait for fetching calendar events from various backends
/// (e.g. Local ICS files, Evolution Data Server, or future COSMIC Accounts).
pub trait CalendarBackend: Send + Sync {
    fn fetch_events<'a>(
        &'a self,
        start: Date,
        end: Date,
    ) -> BoxFuture<'a, Result<Vec<CalendarEvent>, CalendarError>>;
}

/// Backend that loads events and public holidays from local `.ics` files
/// without requiring any system daemon (satisfies Issue #1331).
pub struct LocalIcsBackend {
    file_paths: Vec<PathBuf>,
}

impl LocalIcsBackend {
    pub fn new(file_paths: Vec<PathBuf>) -> Self {
        Self { file_paths }
    }

    /// Discovers default calendar locations such as ~/.local/share/calendars/ and /usr/share/calendar/
    pub fn default_locations() -> Self {
        let mut paths = Vec::new();

        if let Ok(env_path) = std::env::var("COSMIC_CALENDAR_ICS") {
            paths.push(PathBuf::from(env_path));
        }

        // System-wide calendars (/usr/share/calendar)
        let sys_cal_dir = PathBuf::from("/usr/share/calendar");
        if sys_cal_dir.is_dir()
            && let Ok(entries) = std::fs::read_dir(&sys_cal_dir)
        {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("ics") {
                    paths.push(path);
                }
            }
        }

        // User XDG data directory (~/.local/share/calendars and ~/.local/share/cosmic-calendar)
        let base_data_dir = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")));

        if let Some(base) = base_data_dir {
            for dir_name in ["calendars", "cosmic-calendar"] {
                let user_cal_dir = base.join(dir_name);
                if user_cal_dir.is_dir()
                    && let Ok(entries) = std::fs::read_dir(&user_cal_dir)
                {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("ics") {
                            paths.push(path);
                        }
                    }
                }
            }
        }

        Self { file_paths: paths }
    }

    pub fn file_paths(&self) -> &[PathBuf] {
        &self.file_paths
    }

    pub fn add_file(&mut self, path: PathBuf) {
        self.file_paths.push(path);
    }
}

impl CalendarBackend for LocalIcsBackend {
    fn fetch_events<'a>(
        &'a self,
        start: Date,
        end: Date,
    ) -> BoxFuture<'a, Result<Vec<CalendarEvent>, CalendarError>> {
        Box::pin(async move {
            let mut all_events = Vec::new();

            for path in &self.file_paths {
                if path.exists() {
                    match tokio::fs::read_to_string(path).await {
                        Ok(content) => {
                            let parsed = parse_ical_content(&content, start, end)?;
                            all_events.extend(parsed);
                        }
                        Err(err) => {
                            tracing::warn!(?err, path = ?path, "Failed to read calendar file");
                        }
                    }
                }
            }

            all_events.sort_by(|a, b| a.start.cmp(&b.start));
            Ok(all_events)
        })
    }
}

/// Composite backend that merges events from multiple underlying backends
/// (e.g. Local ICS + Evolution Data Server).
pub struct CompositeBackend {
    backends: Vec<std::sync::Arc<dyn CalendarBackend>>,
}

impl CompositeBackend {
    pub fn new(backends: Vec<std::sync::Arc<dyn CalendarBackend>>) -> Self {
        Self { backends }
    }
}

impl CalendarBackend for CompositeBackend {
    fn fetch_events<'a>(
        &'a self,
        start: Date,
        end: Date,
    ) -> BoxFuture<'a, Result<Vec<CalendarEvent>, CalendarError>> {
        Box::pin(async move {
            let mut combined = Vec::new();
            for backend in &self.backends {
                match backend.fetch_events(start, end).await {
                    Ok(events) => combined.extend(events),
                    Err(err) => {
                        tracing::warn!(?err, "Error fetching from backend in composite");
                    }
                }
            }
            combined.sort_by(|a, b| a.start.cmp(&b.start));
            combined.dedup_by(|a, b| !a.id.is_empty() && a.id == b.id);
            Ok(combined)
        })
    }
}

/// Mock backend that generates predictable test events.
pub struct MockBackend;

impl CalendarBackend for MockBackend {
    fn fetch_events<'a>(
        &'a self,
        start: Date,
        _end: Date,
    ) -> BoxFuture<'a, Result<Vec<CalendarEvent>, CalendarError>> {
        Box::pin(async move {
            Ok(crate::event::mock_events_for_month(
                start.year(),
                start.month(),
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[tokio::test]
    async fn test_mock_backend() {
        let backend = MockBackend;
        let events = backend
            .fetch_events(date(2026, 3, 1), date(2026, 3, 31))
            .await
            .unwrap();
        assert!(!events.is_empty());
    }

    #[tokio::test]
    async fn test_local_ics_backend_with_tempfile() {
        let temp_dir = tempfile::tempdir().unwrap();
        let ics_path = temp_dir.path().join("test_holidays.ics");

        let ics_content = "BEGIN:VCALENDAR\r\n\
BEGIN:VEVENT\r\n\
UID:national-holiday-1\r\n\
SUMMARY:National Sovereignty Day\r\n\
DTSTART;VALUE=DATE:20260423\r\n\
DTEND;VALUE=DATE:20260424\r\n\
END:VEVENT\r\n\
END:VCALENDAR";

        std::fs::write(&ics_path, ics_content).unwrap();

        let backend = LocalIcsBackend::new(vec![ics_path]);
        let events = backend
            .fetch_events(date(2026, 4, 1), date(2026, 4, 30))
            .await
            .unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].summary, "National Sovereignty Day");
        assert!(events[0].is_all_day);
    }

    #[tokio::test]
    async fn test_composite_backend() {
        let b1 = std::sync::Arc::new(MockBackend);
        let b2 = std::sync::Arc::new(MockBackend);
        let composite = CompositeBackend::new(vec![b1, b2]);

        let events = composite
            .fetch_events(date(2026, 3, 1), date(2026, 3, 31))
            .await
            .unwrap();

        // 3 mock events each with identical IDs ("mock-1", "mock-2", "mock-3")
        // should be deduplicated to 3 distinct events!
        assert_eq!(events.len(), 3);
        // Verify chronological order
        for w in events.windows(2) {
            assert!(w[0].start <= w[1].start);
        }
    }
}
