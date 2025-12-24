use serde::{Deserialize, Serialize};

/// Represents a log message to be displayed in the UI.
///
/// Log levels:
/// - 0: Info (general information)
/// - 1: Success (successful operations)
/// - 2: Warning (warnings and alerts)
/// - 3: Error (errors and failures)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogMessage {
    pub level: u8,
    pub message: String,
}
