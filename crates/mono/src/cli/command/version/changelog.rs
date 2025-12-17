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

//! Generate the changelog of a version in Markdown format.

use clap::Args;
use semver::Version;
use std::borrow::Cow;

use mono_changeset::Changeset;
use mono_project::version::VersionExt;
use mono_project::Manifest;

use crate::cli::{Command, Result};
use crate::Context;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Generate the changelog of a version in Markdown format.
#[derive(Args, Debug)]
pub struct Arguments {
    /// Version in x.y.z format
    #[arg(value_parser = Version::from_str_with_prefix)]
    version: Option<Version>,
    /// Include version summary.
    #[arg(short, long)]
    summary: bool,
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
        // that are either part of the given version or yet unreleased
        let versions = context.repository.versions()?;
        let mut changeset = Changeset::new(&context.workspace)?;
        for res in versions.commits(self.version.as_ref())? {
            changeset.add(res?)?;
        }

        // Create queue for writing to standard out - since we are particularly
        // careful about line feeds, we collect everything before writing
        let mut queue = Vec::new();
        if self.summary {
            queue.push(Cow::Borrowed(changeset.summary()?));
        }

        // Generate changelog, and append to queue if it's not empty - we also
        // need to support summary-only releases, i.e., pure version bumps
        let changelog = changeset.to_changelog();
        if !changelog.is_empty() {
            queue.push(Cow::Owned(changelog.to_string()));
        }

        // Write everything to standard out
        if !queue.is_empty() {
            println!("{}", queue.join("\n\n"));
        }

        // No errors occurred
        Ok(())
    }
}
