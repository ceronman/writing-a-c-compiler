use std::fs;
use std::path::{Path, PathBuf};

pub struct TempPath(PathBuf);

impl TempPath {
    pub fn new(p: PathBuf) -> Self {
        Self(p)
    }

    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }
}

impl Drop for TempPath {
    fn drop(&mut self) {
        fs::remove_file(&self.0).ok();
    }
}
