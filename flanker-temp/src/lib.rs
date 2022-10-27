//! Helper for working with temporary files.

use std::{fs, process};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use tinyrand::Rand;
use tinyrand_std::thread_rand;

/// A path to a temporary file that will be deleted (if the file was created) when the path
/// is eventually dropped.
#[derive(Debug)]
pub struct TempPath {
    path_buf: PathBuf
}

impl TempPath {
    /// Returns a random path in the system's temp directory.
    pub fn with_extension(extension: &str) -> Self {
        let path_buf = std::env::temp_dir();
        let mut rand = thread_rand();
        let pid = process::id();
        let tid = thread_id::get();
        let random = rand.next_u64();
        let path_buf = path_buf.join(format!("test-{pid}-{tid}-{random:X}.{extension}"));
        Self { path_buf }
    }
}

impl AsRef<Path> for TempPath {
    fn as_ref(&self) -> &Path {
        &self.path_buf
    }
}

impl Drop for TempPath {
    fn drop(&mut self) {
        let _ignore = fs::remove_file(self);
    }
}

#[cfg(test)]
mod tests;