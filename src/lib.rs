// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mint-update-notifier: mint update notifier component model.
//!
//! Real implementation: notifiers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notifier {
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
pub enum MintUpdateNotifierError {
    EmptyNotifierName,
    DuplicateNotifier(String),
    EmptyEventSubject,
}

impl fmt::Display for MintUpdateNotifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNotifierName => write!(f, "notifier name must not be empty"),
            Self::DuplicateNotifier(id) => write!(f, "duplicate notifier {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintUpdateNotifierError {}

pub struct MintUpdateNotifier {
    pub name: String,
    notifiers: BTreeMap<String, Notifier>,
    events: BTreeMap<String, Event>,
}

impl MintUpdateNotifier {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            notifiers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_notifier(&mut self, item: Notifier) -> Result<(), MintUpdateNotifierError> {
        if item.name.trim().is_empty() {
            return Err(MintUpdateNotifierError::EmptyNotifierName);
        }
        let id = item.id.clone();
        if self.notifiers.contains_key(&id) {
            return Err(MintUpdateNotifierError::DuplicateNotifier(id.clone()));
        }
        self.notifiers.insert(id, item);
        Ok(())
    }

    pub fn notifier_count(&self) -> usize {
        self.notifiers.len()
    }

    pub fn get_notifier(&self, id: &str) -> Option<&Notifier> {
        self.notifiers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintUpdateNotifierError> {
        if event.subject.trim().is_empty() {
            return Err(MintUpdateNotifierError::EmptyEventSubject);
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

    fn sample_notifier(id: &str, name: &str) -> Notifier {
        Notifier {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintUpdateNotifier::new("MintUpdateNotifier");
        assert_eq!(m.name, "MintUpdateNotifier");
        assert_eq!(m.notifier_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_notifier() {
        let mut m = MintUpdateNotifier::new("MintUpdateNotifier");
        m.add_notifier(sample_notifier("a1", "Alpha")).unwrap();
        assert_eq!(m.notifier_count(), 1);
        assert!(m.get_notifier("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintUpdateNotifier::new("MintUpdateNotifier");
        assert!(m.add_notifier(sample_notifier("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintUpdateNotifier::new("MintUpdateNotifier");
        m.add_notifier(sample_notifier("x", "One")).unwrap();
        assert!(m.add_notifier(sample_notifier("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintUpdateNotifier::new("MintUpdateNotifier");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
