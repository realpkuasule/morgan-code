use std::path::{Path, PathBuf};

/// File origin - where a file comes from
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOrigin {
    /// File is in Morgan Code's own directories
    MorganCode,

    /// File is in the user's project directory
    Project,

    /// File is in system directories (/usr, /etc, etc.)
    System,

    /// Unknown origin
    Unknown,
}

/// File metadata including origin information
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub origin: FileOrigin,
    pub is_readable: bool,
    pub is_writable: bool,
}

/// Project manager for handling file origin
/// Current working directory is the project directory
pub struct ProjectManager {
    project_root: PathBuf,
    morgan_home: PathBuf,
}

impl ProjectManager {
    /// Create a new project manager
    pub fn new(project_root: PathBuf, morgan_home: PathBuf) -> Self {
        Self {
            project_root,
            morgan_home,
        }
    }

    /// Get file origin information
    pub fn get_file_info(&self, path: &Path) -> FileInfo {
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.project_root.join(path)
        };

        let is_readable = std::fs::metadata(&absolute_path)
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false);

        let is_writable = is_readable && std::fs::metadata(&absolute_path)
            .map(|_m| {
                let mut path = absolute_path.as_path();
                while let Some(parent) = path.parent() {
                    if parent.to_str().unwrap_or("").starts_with("/usr/") ||
                       parent.to_str().unwrap_or("").starts_with("/etc/") ||
                       parent.to_str().unwrap_or("").starts_with("/opt/") {
                        return false;
                    }
                    path = parent;
                }
                true
            })
            .unwrap_or(false);

        let origin = if self.is_in_morgan_code(&absolute_path) {
            FileOrigin::MorganCode
        } else if self.is_in_project(&absolute_path) {
            FileOrigin::Project
        } else if self.is_system_path(&absolute_path) {
            FileOrigin::System
        } else {
            FileOrigin::Unknown
        };

        FileInfo {
            path: absolute_path,
            origin,
            is_readable,
            is_writable,
        }
    }

    /// Check if path is in Morgan Code's directories
    fn is_in_morgan_code(&self, path: &Path) -> bool {
        path.starts_with(&self.morgan_home)
    }

    /// Check if path is in the project directory
    fn is_in_project(&self, path: &Path) -> bool {
        path.starts_with(&self.project_root)
    }

    /// Check if path is in system directories
    fn is_system_path(&self, path: &Path) -> bool {
        let path_str = path.to_str().unwrap_or("");
        path_str.starts_with("/usr/") ||
        path_str.starts_with("/etc/") ||
        path_str.starts_with("/opt/") ||
        path_str.starts_with("/var/") ||
        path_str.starts_with("/lib/") ||
        path_str.starts_with("/bin/")
    }

    /// Get relative path from project root
    pub fn get_relative_path(&self, path: &Path) -> Option<PathBuf> {
        path.strip_prefix(&self.project_root)
            .ok()
            .map(|p| p.to_path_buf())
    }

    /// Get display path with origin label
    pub fn format_path(&self, path: &Path, show_origin: bool) -> String {
        let info = self.get_file_info(path);

        let origin_label = if show_origin {
            match info.origin {
                FileOrigin::MorganCode => "🔧",
                FileOrigin::Project => "📁",
                FileOrigin::System => "🖥️",
                FileOrigin::Unknown => "❓",
            }
        } else {
            ""
        };

        let relative_path = self.get_relative_path(path)
            .unwrap_or_else(|| path.to_path_buf());

        let path_str = relative_path.to_string_lossy().to_string();

        if origin_label.is_empty() {
            path_str
        } else {
            format!("{} {}", origin_label, path_str)
        }
    }
}
