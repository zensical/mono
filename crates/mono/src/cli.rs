// Copyright (c) 2025 Zensical and contributors

// SPDX-License-Identifier: MIT
// Third-party contributions licensed under DCO

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

// ----------------------------------------------------------------------------

//! Command line interface.

use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::Parser;
use std::path::PathBuf;
use std::{env, process};

use mono_project::{Manifest, Workspace};
use mono_repository::Repository;

use crate::Context;

mod command;
mod config;
mod error;

pub use command::{Command, Commands};
pub use config::Config;
pub use error::Result;

// ----------------------------------------------------------------------------
// Constants
// ----------------------------------------------------------------------------

/// Command line styles.
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Command line interface.
#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
#[command(disable_help_subcommand = true)]
#[command(styles = STYLES)]
pub struct Cli {
    /// Working directory.
    #[arg(short, long, value_parser = valid, default_value = ".")]
    pub directory: PathBuf,
    /// Commands.
    #[command(subcommand)]
    command: Commands,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Cli {
    pub fn execute<T>(
        self, repository: Repository, workspace: Workspace<T>, config: Config,
    ) where
        T: Manifest,
    {
        match self
            .command
            .execute(Context { repository, workspace, config })
        {
            Ok(()) => process::exit(0),
            Err(err) => {
                eprintln!("Error: {err}");
                process::exit(1)
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Validates that the given path exists.
fn valid(value: &str) -> Result<PathBuf> {
    let path = PathBuf::from(value);
    path.metadata()?;
    Ok(path)
}
