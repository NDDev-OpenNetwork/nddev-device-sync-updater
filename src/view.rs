//! Portable native-provider response projections. No I/O or UI runtime.
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SystemUpdateAction {
    GdsInstall { platform: String },
    VerifiedApplications,
    AptObserve,
    ToolchainObserve,
    Homebrew,
    Flatpak,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Plan {
    pub schema: u8,
    pub channel: String,
    pub actions: Vec<SystemUpdateAction>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ActionResult {
    pub action: String,
    pub ok: bool,
    #[serde(deserialize_with = "Option::deserialize")]
    pub changed: Option<bool>,
    pub observed: bool,
    pub detail: String,
    pub exit_code: Option<i32>,
    pub state: String,
    pub output_truncated: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateRunSummary {
    pub schema: u8,
    pub started_at: u64,
    pub finished_at: u64,
    pub channel: String,
    pub results: Vec<ActionResult>,
    pub catalog_sequence: Option<u64>,
    pub bootstrap_commit: Option<String>,
}
