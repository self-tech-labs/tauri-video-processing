use crate::error::AppError;
use anyhow::{Result};
use serde::{Deserialize, Serialize};
use std::path::{Path};
use std::process::Command;
use tempfile::NamedTempFile;
use tauri_plugin_process::Command as TauriCommand;
use std::fs; 