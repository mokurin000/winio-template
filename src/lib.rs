//! Root module of the QR code generator app.

pub mod model;

/// Shorthand for `Result<T, color_eyre::Report>`.
pub type Result<T> = std::result::Result<T, color_eyre::Report>;
/// Unique ID for this application.
pub const APP_ID: &str = "io.github.mokurin000.example";

#[cfg(target_os = "android")]
mod android;
