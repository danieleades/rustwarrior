use serde::{Deserialize, Serialize};

/// Coefficients for calculating task urgency
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Coefficients {
    /// The urgency coefficients for task priority
    #[serde(default)]
    pub priority: Priority,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Priority {
    pub p1: f32,
    pub p2: f32,
    pub p3: f32,
}

impl Default for Priority {
    fn default() -> Self {
        Self {
            p1: 6.0,
            p2: 3.9,
            p3: 1.8,
        }
    }
}
