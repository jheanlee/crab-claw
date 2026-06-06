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

use crate::tools::fs::common::FsConfig;
use std::sync::LazyLock;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod api;
mod conversation_message;
mod core;
mod interface;
mod llm;
mod messaging;
mod tools;

pub static FS_CONFIG: LazyLock<FsConfig> = LazyLock::new(|| FsConfig {
    read_whitelist: Default::default(),
    write_whitelist: Default::default(),
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    Ok(())
}
