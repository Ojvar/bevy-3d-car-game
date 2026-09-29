//! Shared types: constants, components, and resources.

pub mod components;
pub mod constants;
pub mod resources;

use std::path::PathBuf;

const APP_DIR: &str = "bevy-card-rider";

/// Path of a per-user save file (`$XDG_DATA_HOME/bevy-card-rider/<file>`),
/// falling back to the working directory when no data dir is known.
pub fn data_file(file: &str) -> PathBuf {
    let data_dir = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("APPDATA").map(PathBuf::from))
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")));
    match data_dir {
        Some(dir) => dir.join(APP_DIR).join(file),
        None => PathBuf::from(file),
    }
}

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
