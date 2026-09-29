//! Shared types: constants, components, and resources.

pub mod components;
pub mod constants;
pub mod resources;

use std::path::PathBuf;

/// Mirrors Bevy's asset root lookup: `BEVY_ASSET_ROOT`, then `CARGO_MANIFEST_DIR`
/// (set by `cargo run`), then the executable's folder.
pub fn assets_dir() -> Option<PathBuf> {
    std::env::var_os("BEVY_ASSET_ROOT")
        .or_else(|| std::env::var_os("CARGO_MANIFEST_DIR"))
        .map(PathBuf::from)
        .or_else(|| {
            std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(std::path::Path::to_path_buf))
        })
        .map(|root| root.join("assets"))
}
