use std::path::PathBuf;
use url::Url;

use uv_normalize::PackageName;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    WheelFilename(#[from] uv_distribution_filename::WheelFilenameError),

    #[error("Could not extract path segments from URL: {0}")]
    MissingPathSegments(String),

    #[error("Could not extract wheel filename from path: {}", _0.display())]
    MissingWheelFilename(PathBuf),

    #[error("Distribution not found at: {0}")]
    NotFound(Url),

    #[error("Requested package name `{0}` does not match `{1}` in the distribution filename: {2}")]
    PackageNameMismatch(PackageName, PackageName, String),
}
