use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A task to be completed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    uuid: Uuid,
    created: DateTime<Utc>,
    description: String,
}

impl Task {
    /// Create a new [`Task`]
    #[must_use]
    pub fn new(description: String) -> Self {
        let uuid = Uuid::new_v4();
        let created = Utc::now();
        Self {
            uuid,
            created,
            description,
        }
    }

    /// The description of the [`Task`]
    #[must_use]
    pub const fn description(&self) -> &String {
        &self.description
    }
}
