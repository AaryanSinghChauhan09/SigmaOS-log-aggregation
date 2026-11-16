// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-log-aggregation: log aggregation component model.
//!
//! Real implementation: logs, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Log {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogAggregationError {
    EmptyLogName,
    DuplicateLog(String),
    EmptyEventSubject,
}

impl fmt::Display for LogAggregationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLogName => write!(f, "log name must not be empty"),
            Self::DuplicateLog(id) => write!(f, "duplicate log {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LogAggregationError {}

pub struct LogAggregation {
    pub name: String,
    logs: BTreeMap<String, Log>,
    events: BTreeMap<String, Event>,
}

impl LogAggregation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            logs: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_log(&mut self, item: Log) -> Result<(), LogAggregationError> {
        if item.name.trim().is_empty() {
            return Err(LogAggregationError::EmptyLogName);
        }
        let id = item.id.clone();
        if self.logs.contains_key(&id) {
            return Err(LogAggregationError::DuplicateLog(id.clone()));
        }
        self.logs.insert(id, item);
        Ok(())
    }

    pub fn log_count(&self) -> usize {
        self.logs.len()
    }

    pub fn get_log(&self, id: &str) -> Option<&Log> {
        self.logs.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LogAggregationError> {
        if event.subject.trim().is_empty() {
            return Err(LogAggregationError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_log(id: &str, name: &str) -> Log {
        Log {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = LogAggregation::new("LogAggregation");
        assert_eq!(m.name, "LogAggregation");
        assert_eq!(m.log_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_log() {
        let mut m = LogAggregation::new("LogAggregation");
        m.add_log(sample_log("a1", "Alpha")).unwrap();
        assert_eq!(m.log_count(), 1);
        assert!(m.get_log("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = LogAggregation::new("LogAggregation");
        assert!(m.add_log(sample_log("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = LogAggregation::new("LogAggregation");
        m.add_log(sample_log("x", "One")).unwrap();
        assert!(m.add_log(sample_log("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = LogAggregation::new("LogAggregation");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
