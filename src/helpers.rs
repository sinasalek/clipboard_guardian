use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use std::{env};

// Get absolute path to the "target" directory ("build" dir)
pub fn get_target_dir() -> PathBuf {
    let bin = env::current_exe().expect("exe path");
    let mut target_dir = PathBuf::from(bin.parent().expect("bin parent"));
    while target_dir.file_name() != Some(OsStr::new("target")) {
        target_dir.pop();
    }
    target_dir
}
// Get absolute path to the project's top dir, given target dir
pub fn get_top_dir<'a>(target_dir: &'a Path) -> &'a Path {
    target_dir.parent().expect("target parent")
}