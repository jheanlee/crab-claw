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

use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    InvalidParameter(String),
    IoError(tokio::io::Error),
    JsonError(serde_json::Error),
    NonUTF8PathName,
    WhitelistViolation,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParameter(error_message) => write!(f, "InvalidParameter {error_message}"),
            Self::IoError(error) => write!(f, "IoError: {error}"),
            Self::JsonError(error) => write!(f, "JsonError: {error}"),
            Self::NonUTF8PathName => write!(f, "NonUTF8PathName"),
            Self::WhitelistViolation => write!(f, "Action denied (whitelist)"),
        }
    }
}

impl std::error::Error for Error {}

impl From<tokio::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(error)
    }
}
