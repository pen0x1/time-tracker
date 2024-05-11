#[derive(Debug)]
enum TimeManagerError {
    EntryNotFound,
    InvalidOperation(String),
}

use chrono::{NaiveDate, NaiveDateTime, Duration, Local};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
struct TimeEntry {
    project_id: u32,
    start: NaiveDateTime,
    duration: Duration, // Changed to store duration directly
}

struct TimeManager {
    entries: Vec<TimeEntry>,
    env_project_hours: HashMap<u32, f32>, // Using f32 for reduced memory usage
}

impl TimeManager {
    fn new() -> Self {
        dotenv::dotenv().ok();
        let mut env_project_hours = HashMap::new();
        for (key, value) in env::vars() {
            if key.ends_with("_HOURS") {
                if let Ok(id) = key.replace("_HOURS", "").parse::<u32>() {
                    if let Ok(hours) = value.parse::<f32>() { // Parsing as f32
                        env_project_hours.insert(id, hours);
                    }
                }
            }
        }

        TimeManager {
            entries: Vec::new(),
            env_project_hours,
        }
    }

    fn add_entry(&mut self, project_id: u32, start: NaiveDateTime, end: Option<NaiveDateTime>) {
        let duration = end.unwrap_or_else(|| Local::now().naive_local()) - start;
        let entry = TimeEntry { project_id, start, duration };
        self.entries.push(entry);
    }

    fn update_entry(&mut self, entry_index: usize, new_start: NaiveDateTime, new_end: Option<NaiveDateTime>) -> Result<(), TimeManagerError> {
        if let Some(entry) = self.entries.get_mut(entry_index) {
            entry.start = new_start;
            entry.duration = new_end.map_or_else(|| Local::now().naive_local() - new_start, |end| end - new_start);
            Ok(())
        } else {
            Err(TimeManagerError::EntryNotFound)
        }
    }

    fn calculate_project_time(&self, project_id: u32) -> Duration {
        self.entries.iter()
            .filter(|e| e.project_id == project_id)
            .map(|e| e.duration)
            .fold(Duration::zero(), |acc, d| acc + d)
    }

    // Switched to returning references for better memory efficiency.
    fn list_entries_by_project(&self, project_id: u32) -> Vec<&TimeEntry> {
        self.entries.iter()
            .filter(|entry| entry.project_id == project_id)
            .collect()
    }
}

fn main() {
    // The rest of your main function remains unchanged.
    // Note: Adjustments might be needed depending on the use of returned values from list_entries_by_project, as it now returns references.
}