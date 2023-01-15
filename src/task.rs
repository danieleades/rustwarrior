use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use self::priority::Priority;

mod priority;

/// A task to be completed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    uuid: Uuid,
    created: DateTime<Utc>,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    priority: Option<Priority>,
}

impl Task {
    /// Create a new [`Task`]
    #[must_use]
    pub fn new(description: String) -> Self {
        let uuid = Uuid::new_v4();
        let created = Utc::now();
        let priority = None;
        Self {
            uuid,
            created,
            description,
            priority,
        }
    }

    /// Set the priority of the [`Task`]
    #[must_use]
    pub const fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// The [`Task`] priority
    #[must_use]
    pub const fn priority(&self) -> Option<Priority> {
        self.priority
    }

    /// The description of the [`Task`]
    #[must_use]
    pub const fn description(&self) -> &String {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{DateTime, Utc};
    use test_case::test_case;
    use uuid::uuid;

    use super::{Priority, Task};

    #[test_case(
        r#"{"uuid":"ee00fce2-f349-42b1-969e-17d4c6c612f5","created":"2023-01-14T09:57:04.275194707Z","description":"some made up task"}"#
        => Task {uuid: uuid!("ee00fce2-f349-42b1-969e-17d4c6c612f5"), created: DateTime::<Utc>::from_str("2023-01-14T09:57:04.275194707Z").unwrap(), description: "some made up task".to_string(), priority: None}
        ; "minimal"
    )]
    #[test_case(
        r#"{"uuid":"ee00fce2-f349-42b1-969e-17d4c6c612f5","created":"2023-01-14T09:57:04.275194707Z","description":"some made up task", "priority": 2}"#
        => Task {
            uuid: uuid!("ee00fce2-f349-42b1-969e-17d4c6c612f5"),
            created: DateTime::<Utc>::from_str("2023-01-14T09:57:04.275194707Z").unwrap(),
            description: "some made up task".to_string(),
            priority: Some(Priority::Two)
        }
        ; "priority"
    )]
    fn deserialise(input: &str) -> Task {
        serde_json::from_str(input).unwrap()
    }
}
