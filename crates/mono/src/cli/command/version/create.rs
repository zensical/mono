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

//! Create a new version and update all packages.

use clap::Args;
use cliclack::{confirm, outro, select};
use console::style;
use std::io::Write;
use std::{fs, process};
use tempfile::NamedTempFile;

use mono_changeset::Changeset;
use mono_project::version::VersionExt;
use mono_project::Manifest;

use crate::cli::{Command, Result};
use crate::Context;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Create a new version and update all packages.
#[derive(Args, Debug)]
pub struct Arguments {
    /// Use visual editor for release notes.
    #[arg(short, long)]
    visual: bool,
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
        // Resolve versions and create changeset, then determine all commits
        // that were added after the latest version was released
        let versions = context.repository.versions()?;
        let mut changeset = Changeset::new(&context.workspace)?;
        for res in versions.commits(None)? {
            changeset.add(res?)?;
        }

        // Obtain version increments, which denote which packages have changed,
        // and abort immediately if there are no changes that require a release
        let mut increments = changeset.increments().to_vec();
        if increments.iter().all(Option::is_none) {
            eprintln!("Nothing to release");
            return Ok(());
        }

        // Ensure working directory is clean
        if !context.repository.is_clean()? {
            eprintln!("Working directory contains changes");
            return Ok(());
        }

        // Ensure we're on the default branch
        if !context.repository.on_default_branch()? {
            eprintln!("Not on default branch");
            return Ok(());
        }

        // Prompt the user whether a new version should be created
        if !confirm("Create new version?")
            .initial_value(true)
            .interact()?
        {
            return Ok(());
        }

        // Traverse dependents in topological order, to let the user review
        // version increment suggestions in lock-step for choosing
        let dependents = context.workspace.dependents()?;
        dependents.bump(&mut increments, |suggestion| {
            let project = suggestion.project();
            let increments = suggestion.increments();

            // Retrieve namd and version of project - only packages are allowed
            // to be dependents, which means name and version definitely exist
            let name = project.name().expect("invariant");
            let version = project.version().expect("invariant");

            // Create select builder, and add all possible version increments,
            // as depending on the changes, multiple increments are possible
            let mut builder =
                increments.iter().fold(select(name), |builder, &bump| {
                    if let Some(next) = bump {
                        builder.item(Some(next), version.bump(next), next)
                    } else {
                        builder.item(None, version, "current")
                    }
                });

            // Prompt the user to select a version increment
            Ok(builder.interact()?)
        })?;

        // Denote completion of prompt to the user
        outro(style("Versions selected").dim())?;

        // Determine sink - @todo make sure there is only one?
        let Some(sink) = dependents.sinks().next() else {
            eprintln!("No canonical crate found");
            return Ok(());
        };

        // Extract version of sink
        let version = dependents[sink].version().expect("invariant");
        let version = if let Some(b) = increments[sink] {
            version.bump(b)
        } else {
            version.clone()
        };

        // Create a branch and bump all related files
        context.repository.branch(format!("release/v{version}"))?;
        context.workspace.bump(&increments)?;

        // Create commit message with summary and body
        let summary = prompt_commit_message(self.visual)?;
        let message = format!("chore: release v{version}\n\n{summary}");

        // Add all files and commit
        context.repository.add("*")?;
        context.repository.commit(message)?;

        // No errors occurred
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Prompts the user to enter a commit message.
fn prompt_commit_message(visual: bool) -> Result<String> {
    let mut temp = NamedTempFile::new()?;
    writeln!(temp, "## Summary\n\n...\n\n### Highlights\n\n- ...")?;

    // Get editor from environment or use default
    let editor = if visual {
        std::env::var("VISUAL").unwrap_or_else(|_| "vim".to_string())
    } else {
        std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string())
    };

    // Open editor, using `--wait` for VS Code
    let mut cmd = process::Command::new(&editor);
    cmd.arg(temp.path());
    if editor == "code" {
        cmd.arg("--wait");
    }

    // Check status after the user finished editing
    let status = cmd.status()?;
    if !status.success() {
        process::exit(status.code().unwrap_or(1));
    }

    // Ensure message is not empty
    let message = fs::read_to_string(temp.path())?;
    if message.is_empty() {
        eprintln!("Commit message cannot be empty");
        process::exit(1);
    }

    // Return message
    Ok(message)
}
