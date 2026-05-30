use crate::tools::error::Error;
use globset::GlobSet;
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;

pub struct FsConfig {
    pub read_whitelist: RwLock<GlobSet>,
    pub write_whitelist: RwLock<GlobSet>,
}

pub struct FsUtils {}

impl FsUtils {
    pub fn get_absolute_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, Error> {
        if path.as_ref().is_absolute() {
            Ok(PathBuf::from(path.as_ref()))
        } else {
            Ok(PathBuf::from(path.as_ref().canonicalize()?))
        }
    }
}
