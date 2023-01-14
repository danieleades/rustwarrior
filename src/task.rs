use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    id: usize,
    uuid: Uuid,
    created: DateTime<Utc>,
    description: String,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        let uuid = Uuid::new_v4();
        let created = Utc::now();
        Self {
            id,
            uuid,
            created,
            description,
        }
    }

    pub const fn id(&self) -> usize {
        self.id
    }

    pub const fn description(&self) -> &String {
        &self.description
    }
}
