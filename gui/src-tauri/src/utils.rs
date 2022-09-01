#[cfg(target_os = "macos")]
pub fn file_manager() -> &'static str {
    "open"
}
#[cfg(target_os = "windows")]
pub fn file_manager() -> &'static str {
    "explorer"
}
#[cfg(unix)]
pub fn file_manager() -> &'static str {
    "xdg-open"
}
