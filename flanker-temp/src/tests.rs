use std::fs;
use std::path::PathBuf;
use crate::TempPath;

#[test]
fn lifecycle_uninitialised() {
    let temp = TempPath::with_extension("tmp");
    // temp file is lazily created
    assert!(!temp.as_ref().exists());

    // dropping an uninitialised TempFile does nothing
    let path = PathBuf::from(temp.as_ref());
    drop(temp);
    assert!(!path.exists());
}

#[test]
fn lifecycle_initialised() {
    let temp = TempPath::with_extension("tmp");
    // temp file is lazily created
    assert!(!temp.as_ref().exists());

    // need to write to it for it be created
    fs::write(temp.as_ref(), "foo").unwrap();
    assert!(temp.as_ref().exists());

    // dropping a TempPath deletes the underlying file
    let path = PathBuf::from(temp.as_ref());
    drop(temp);
    assert!(!path.exists());
}

#[test]
fn implements_debug() {
    let temp = TempPath::with_extension("tmp");
    let s = format!("{temp:?}");
    assert!(s.contains("TempPath"));
    assert!(s.contains(temp.path_buf.to_string_lossy().as_ref()));
}