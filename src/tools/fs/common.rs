/*
 * Copyright 2026 Jhe-An Lee
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
