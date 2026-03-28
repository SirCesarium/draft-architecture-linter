//! Terminal UI and visual feedback configuration.

use serde::{Deserialize, Serialize};

/// Visual feedback configuration for terminal reporting.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UIConfig {
    /// Line threshold to trigger the 'lemon' warning.
    #[serde(default = "default_lemon_threshold")]
    pub lemon_threshold: usize,
    /// Line threshold to trigger the 'bitter' error status.
    #[serde(default = "default_bitter_threshold")]
    pub bitter_threshold: usize,
}

pub const fn default_lemon_threshold() -> usize {
    200
}
pub const fn default_bitter_threshold() -> usize {
    400
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            lemon_threshold: 200,
            bitter_threshold: 400,
        }
    }
}
