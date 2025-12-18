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

//! Validate a commit message.

use clap::{ArgGroup, Args};
use cliclack::{confirm, input, outro};
use console::style;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, process};

use mono_changeset::change::{Error, Kind};
use mono_changeset::Change;
use mono_project::Manifest;

use crate::cli::{Command, Result};
use crate::Context;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Validate a commit message.
#[derive(Args, Debug)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["summary", "file", "id"])
))]
pub struct Arguments {
    /// Commit message summary.
    summary: Option<String>,
    /// Commit message file.
    #[arg(short, long)]
    file: Option<PathBuf>,
    /// Commit identifier.
    #[arg(short, long)]
    id: Option<String>,
    /// Prompt to add missing information.
    #[arg(short, long)]
    prompt: bool,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<T> Command<T> for Arguments
where
    T: Manifest,
{
    /// Executes the command.
    fn execute(&self, context: Context<T>) -> Result {
        // Validate a commit identifier
        if let Some(id) = &self.id {
            let commit = context.repository.find(id)?;
            if parse_summary(commit.summary()).is_none() {
                process::exit(1);
            }

        // Validate a commit message summary
        } else if let Some(summary) = &self.summary {
            if parse_summary(summary).is_none() {
                process::exit(1);
            }

        // Validate a commit message file
        } else {
            let path = self.file.as_ref().expect("invariant");
            let message = fs::read_to_string(path)?;

            // Retrieve first line, and parse as summary
            let summary = message.lines().next().unwrap_or_default();
            let Some(change) = parse_summary(summary) else {
                process::exit(1)
            };

            // Prompt the user for missing information
            let references = change.references();
            if self.prompt && references.is_empty() {
                if confirm("Is this commit related to an issue?")
                    .initial_value(true)
                    .interact()?
                {
                    // Prompt the number of the commit
                    let num: u32 = input("What's the number of the issue?")
                        .placeholder("  e.g. 123")
                        .interact()?;

                    // Append the reference to the first line
                    let mut lines: Vec<&str> = message.lines().collect();
                    let first = lines.get_mut(0).expect("invariant");
                    let slice = format!("{first} (#{num})");
                    *first = &slice;

                    // Join lines and write back to file
                    let new_message = lines.join("\n");
                    fs::write(path, new_message)?;

                    // Denote completion of prompt to the user
                    outro(format!(
                        "{} {}",
                        style(format!("(#{num})")),
                        style("added to commit summary").dim()
                    ))?;

                // Issue is not related to a commit
                } else {
                    outro(style("Nothing added to commit summary").dim())?;
                }
            }
        }

        // No errors occurred
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Parses and validates the format of the given commit summary.
fn parse_summary(summary: &str) -> Option<Change> {
    let err = match Change::from_str(summary) {
        Ok(change) => return Some(change),
        Err(err) => err,
    };

    // Write to standard error and add a useful hint, describing the error type
    // and being as specific as possible, to help the user fix the issue
    eprintln!("{} {}", style("✘").red(), summary);
    match err {
        Error::Format => {
            eprintln!(
                "  {} {}",
                style("Summary must be in the format").dim(),
                style("<type>: <description>")
            );
        }
        Error::Kind => {
            eprintln!(
                "  {} {}",
                style("Supported type:").dim(),
                Kind::VALUES.map(|kind| kind.to_string()).join(", ")
            );
        }
        err => {
            eprintln!(
                "  {} {}", // fmt
                style("Commit").dim(),
                style(err).dim()
            );
        }
    }

    // Return nothing
    None
}
