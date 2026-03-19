use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

/// A task project, optionally organized into subprojects using dot-separated
/// segments.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub struct Project(String);

impl Project {
    /// Return the project string as stored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check whether this project belongs to the provided parent project.
    #[must_use]
    pub fn is_in(&self, project: &Self) -> bool {
        self == project
            || self
                .0
                .strip_prefix(project.as_str())
                .is_some_and(|suffix| suffix.starts_with('.'))
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Project> for String {
    fn from(value: Project) -> Self {
        value.0
    }
}

impl TryFrom<String> for Project {
    type Error = ParseProjectError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl FromStr for Project {
    type Err = ParseProjectError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        validate(value)?;
        Ok(Self(value.to_owned()))
    }
}

/// An error returned when a project path is invalid.
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum ParseProjectError {
    /// The project path was empty.
    #[error("project must not be empty")]
    Empty,
    /// One of the path segments was empty.
    #[error("project segments must not be empty")]
    EmptySegment,
    /// One of the path segments had surrounding whitespace.
    #[error("project segments must not have leading or trailing whitespace")]
    SegmentWhitespace,
}

fn validate(value: &str) -> Result<(), ParseProjectError> {
    if value.is_empty() {
        return Err(ParseProjectError::Empty);
    }

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(ParseProjectError::EmptySegment);
        }
        if segment.trim() != segment {
            return Err(ParseProjectError::SegmentWhitespace);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use test_case::test_case;

    use super::{ParseProjectError, Project};

    #[test_case("" => Err(ParseProjectError::Empty); "empty")]
    #[test_case(".Home" => Err(ParseProjectError::EmptySegment); "leading dot")]
    #[test_case("Home." => Err(ParseProjectError::EmptySegment); "trailing dot")]
    #[test_case("Home..Garden" => Err(ParseProjectError::EmptySegment); "double dot")]
    #[test_case(" Home" => Err(ParseProjectError::SegmentWhitespace); "leading whitespace")]
    #[test_case("Home. Garden" => Err(ParseProjectError::SegmentWhitespace); "segment whitespace")]
    #[test_case("Home.Garden" => Ok(Project::from_str("Home.Garden").unwrap()); "valid")]
    fn parse(input: &str) -> Result<Project, ParseProjectError> {
        Project::from_str(input)
    }

    #[test]
    fn subproject_matching() {
        let project = Project::from_str("Home.Garden.Front").unwrap();

        assert!(project.is_in(&Project::from_str("Home").unwrap()));
        assert!(project.is_in(&Project::from_str("Home.Garden").unwrap()));
        assert!(!project.is_in(&Project::from_str("Hom").unwrap()));
        assert!(!project.is_in(&Project::from_str("Home.Gardens").unwrap()));
        assert!(project.is_in(&Project::from_str("Home.Garden.Front").unwrap()));
    }
}
