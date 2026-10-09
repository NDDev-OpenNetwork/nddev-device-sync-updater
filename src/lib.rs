//! Read-only views of the signed-release/native update owner.
//! A plan is policy intent, never proof that an update ran or is available.

pub mod view;
pub use view::*;
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
mod native;
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub use native::Updater;

/// Canonical compiled-in runtime manifest for this integration.
pub const MANIFEST: &str = include_str!("../contracts/module.json");
