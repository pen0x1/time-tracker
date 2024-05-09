#[derive(Debug)]
enum TimeManagerError {
    EntryNotFound,
    InvalidOperation(String),
}

use chrono::{NaiveDate, NaiveDateTime, Duration, Local};
use std::collections::HashMap;
use std::env;
use std::fmt;

#[derive(Debug, Clone)]
struct TimeEntry {
    project_id: u32,
    start: NaiveDateTime,
    end: Option<NaiveDateTime>,
}

struct TimeManager {
    entries: Vec<TimeEntry>,
    env_project_hours: HashMap<u32, f64>,
}

impl TimeManager {
    fn new() -> Self {
        dotenv::dotenv().ok();
        let mut env_project_hours = HashMap::new();
        for (key, value) in env::vars() {
            if key.ends_with("_HOURS") {
                if let Ok(id) = key.replace("_HOURS", "").parse::<u32>() {
                    if let Ok(hours) = value.parse::<f64>() {
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
        let entry = TimeEntry {
            project_id,
            start,
            end,
        };
        self.entries.push(entry);
    }

    fn update_entry(&mut self, entry_index: usize, new_start: NaiveDateTime, new_end: Option<NaiveDateTime>) -> Result<(), TimeManagerError> {
        if let Some(entry) = self.entries.get_mut(entry_index) {
            entry.start = new_start;
            entry.end = new_end;
            Ok(())
        } else {
            Err(TimeManagerError::EntryNotFound)
        }
    }

    fn delete_entry(&mut self, entry_index: usize) -> Result<(), TimeManagerError> {
        if entry_index < self.entries.len() {
            self.entries.remove(entry_index);
            Ok(())
        } else {
            Err(TimeManagerError::EntryNotFound)
        }
    }

    fn calculate_project_time(&self, project_id: u32) -> Duration {
        self.entries.iter()
            .filter(|e| e.project_id == project_id)
            .map(|e| e.end.unwrap_or(Local::now().naive_local()) - e.start)
            .fold(Duration::zero(), |acc, d| acc + d)
    }

    fn summarize_time(&self, from_date: NaiveDate, to_date: NaiveDate) -> HashMap<u32, Duration> {
        self.entries.iter()
            .filter(|e| e.start.date() >= from_date && e.start.date() <= to_date)
            .fold(HashMap::new(), |mut acc, e| {
                let duration = e.end.unwrap_or(Local::now().naive_local()) - e.start;
                *acc.entry(e.project_id).or_insert(Duration::zero()) += duration;
                acc
            })
    }

    // New function to list entries by project
    fn list_entries_by_project(&self, project_id: u32) -> Vec<TimeEntry> {
        self.entries.iter()
            .filter(|entry| entry.project_id == project_id)
            .cloned() // Clone the filtered entries to return a Vec<TimeEntry>
            .collect()
    }
}

fn main() {
    let mut manager = TimeManager::new();
    let start = NaiveDate::from_ymd(2023, 9, 15).and_hms(9, 0, 0);
    let end = Some(NaiveDate::from_ymd(2023, 9, 15).and_hms(17, 30, 0));
    
    manager.add_entry(1, start, end);
    manager.add_entry(2, start, end); // Added for demonstration
    manager.add_entry(1, NaiveDate::from_ymd(2023, 9, 16).and_hms(9, 0, 0), Some(NaiveDate::from_ymd(2023, 9, 16).and_hms(17, 30, 0))); // Additional entry for project 1

    let project_entries = manager.list_entries_by_project(1);
    println!("Entries for project 1: {:?}", project_entries);

    let project_time = manager.calculate_project_time(1);
    println!("Time spent on project 1: {:?}", project_time);

    let summary = manager.summarize_time(NaiveDate::from_ymd(2023, 9, 1), NaiveDate::from_ymd(2023, 9, 30));
    println!("Summary: {:?}", summary);
}