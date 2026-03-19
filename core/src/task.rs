use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use self::{priority::Priority, project::Project};

mod priority;
mod project;

/// A task to be completed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    uuid: Uuid,
    created: DateTime<Utc>,
    description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    completed: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    priority: Option<Priority>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    project: Option<Project>,
}

impl Task {
    /// Create a new [`Task`]
    #[must_use]
    pub fn new(description: String) -> Self {
        let uuid = Uuid::new_v4();
        let created = Utc::now();
        let completed = None;
        let priority = None;
        let project = None;
        Self {
            uuid,
            created,
            description,
            completed,
            priority,
            project,
        }
    }

    /// The UUID of the [`Task`]
    #[must_use]
    pub const fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// The creation timestamp of the [`Task`]
    #[must_use]
    pub const fn created(&self) -> DateTime<Utc> {
        self.created
    }

    /// The description of the [`Task`]
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set the project of the [`Task`]
    #[must_use]
    pub fn with_project(mut self, project: Project) -> Self {
        self.project = Some(project);
        self
    }

    /// The [`Task`] project
    #[must_use]
    pub const fn project(&self) -> Option<&Project> {
        self.project.as_ref()
    }

    /// Set the project of this task
    pub fn set_project(&mut self, project: Option<Project>) {
        self.project = project;
    }

    /// Check whether the task belongs to the provided project or one of its
    /// subprojects
    #[must_use]
    pub fn matches_project(&self, project: &Project) -> bool {
        self.project
            .as_ref()
            .is_some_and(|task_project| task_project.is_in(project))
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

    /// Set the priority of this task
    pub const fn set_priority(&mut self, priority: Option<Priority>) {
        self.priority = priority;
    }

    /// Check if the task is completed
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        self.completed.is_some()
    }

    /// Get the completion timestamp if the task is completed
    #[must_use]
    pub const fn completed(&self) -> Option<DateTime<Utc>> {
        self.completed
    }

    /// Mark the task as completed
    pub fn mark_completed(&mut self) {
        if self.completed.is_none() {
            self.completed = Some(Utc::now());
        }
    }

    /// Mark the task as active (not completed)
    pub const fn mark_active(&mut self) {
        self.completed = None;
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{DateTime, Utc};
    use test_case::test_case;
    use uuid::uuid;

    use super::{Priority, Project, Task};

    #[test_case(
        r#"{"uuid":"ee00fce2-f349-42b1-969e-17d4c6c612f5","created":"2023-01-14T09:57:04.275194707Z","description":"some made up task"}"#
        => Task {uuid: uuid!("ee00fce2-f349-42b1-969e-17d4c6c612f5"), created: DateTime::<Utc>::from_str("2023-01-14T09:57:04.275194707Z").unwrap(), description: "some made up task".to_string(), completed: None, priority: None, project: None}
        ; "minimal"
    )]
    #[test_case(
        r#"{"uuid":"ee00fce2-f349-42b1-969e-17d4c6c612f5","created":"2023-01-14T09:57:04.275194707Z","description":"some made up task", "priority": 2}"#
        => Task {
            uuid: uuid!("ee00fce2-f349-42b1-969e-17d4c6c612f5"),
            created: DateTime::<Utc>::from_str("2023-01-14T09:57:04.275194707Z").unwrap(),
            description: "some made up task".to_string(),
            completed: None,
            priority: Some(Priority::Two),
            project: None,
        }
        ; "priority"
    )]
    #[test_case(
        r#"{"uuid":"ee00fce2-f349-42b1-969e-17d4c6c612f5","created":"2023-01-14T09:57:04.275194707Z","description":"some made up task", "project": "Home.Garden"}"#
        => Task {
            uuid: uuid!("ee00fce2-f349-42b1-969e-17d4c6c612f5"),
            created: DateTime::<Utc>::from_str("2023-01-14T09:57:04.275194707Z").unwrap(),
            description: "some made up task".to_string(),
            completed: None,
            priority: None,
            project: Some(Project::from_str("Home.Garden").unwrap()),
        }
        ; "project"
    )]
    fn deserialise(input: &str) -> Task {
        serde_json::from_str(input).unwrap()
    }

    #[test]
    fn priority() {
        let task = Task::new("description".to_string());
        assert!(task.priority().is_none());

        let task2 = Task::new("description".to_string()).with_priority(Priority::Three);
        assert!(matches!(task2.priority(), Some(Priority::Three)));
    }

    #[test]
    fn project() {
        let task = Task::new("description".to_string());
        assert!(task.project().is_none());

        let task2 = Task::new("description".to_string())
            .with_project(Project::from_str("Home.Garden").unwrap());
        assert_eq!(task2.project().map(Project::as_str), Some("Home.Garden"));
        assert!(task2.matches_project(&Project::from_str("Home").unwrap()));
        assert!(task2.matches_project(&Project::from_str("Home.Garden").unwrap()));
        assert!(!task2.matches_project(&Project::from_str("Work").unwrap()));
    }
}
