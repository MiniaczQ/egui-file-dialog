use std::fs;
use std::path::{Path, PathBuf};

/// Wrapper above directories::UserDirs.
/// Currently only used to canonicalize the paths.
#[derive(Default, Clone)]
pub struct UserDirectories {
    home_dir: Option<PathBuf>,

    audio_dir: Option<PathBuf>,
    desktop_dir: Option<PathBuf>,
    document_dir: Option<PathBuf>,
    download_dir: Option<PathBuf>,
    picture_dir: Option<PathBuf>,
    video_dir: Option<PathBuf>,
}

impl UserDirectories {
    /// Creates a new UserDirectories object.
    /// Returns None if no valid home directory path could be retrieved from the operating system.
    pub fn new() -> Option<Self> {
        if let Some(dirs) = directories::UserDirs::new() {
            return Some(Self {
                home_dir: Self::canonicalize(Some(dirs.home_dir())),

                audio_dir: Self::canonicalize(dirs.audio_dir()),
                desktop_dir: Self::canonicalize(dirs.desktop_dir()),
                document_dir: Self::canonicalize(dirs.document_dir()),
                download_dir: Self::canonicalize(dirs.download_dir()),
                picture_dir: Self::canonicalize(dirs.picture_dir()),
                video_dir: Self::canonicalize(dirs.video_dir()),
            });
        }

        None
    }

    pub fn home_dir(&self) -> Option<&Path> {
        self.home_dir.as_deref()
    }

    pub fn audio_dir(&self) -> Option<&Path> {
        self.audio_dir.as_deref()
    }

    pub fn desktop_dir(&self) -> Option<&Path> {
        self.desktop_dir.as_deref()
    }

    pub fn document_dir(&self) -> Option<&Path> {
        self.document_dir.as_deref()
    }

    pub fn download_dir(&self) -> Option<&Path> {
        self.download_dir.as_deref()
    }

    pub fn picture_dir(&self) -> Option<&Path> {
        self.picture_dir.as_deref()
    }

    pub fn video_dir(&self) -> Option<&Path> {
        self.video_dir.as_deref()
    }

    /// Canonicalizes the given paths. Returns None if an error occurred.
    fn canonicalize(path: Option<&Path>) -> Option<PathBuf> {
        if let Some(path) = path {
            return match fs::canonicalize(path) {
                Ok(p) => Some(p),
                Err(_) => None,
            };
        }

        None
    }
}
