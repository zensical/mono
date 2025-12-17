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

//! List the names of changed packages in topological order.

use clap::Args;
use semver::Version;
use std::cmp;

use mono_changeset::Changeset;
use mono_project::version::{Increment, VersionExt};
use mono_project::Manifest;

use crate::cli::{Command, Result};
use crate::Context;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// List the names of changed packages in topological order.
#[derive(Args, Debug)]
pub struct Arguments {
    /// Version in x.y.z format
    #[arg(value_parser = Version::from_str_with_prefix)]
    version: Option<Version>,
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

        // Obtain version increments, which denote which packages have changed,
        // and if a version is given, ensure that all packages that were bumped
        // in the given version are marked as changed, since there might be
        // transitive changes that only affect dependencies
        let mut increments = changeset.increments().to_vec();
        if self.version.is_some() {
            let scopes = changeset.scopes();
            if let Some(head) = changeset.revisions().first() {
                let commit = head.commit();

                // Traverse all deltas in the commit, and mark corresponding
                // scopes as changed with at least a patch increment
                for delta in commit.deltas()? {
                    if let Some(node) = scopes.get(delta.path()) {
                        increments[node] =
                            cmp::max(increments[node], Some(Increment::Patch));
                    }
                }
            }
        }

        // Traverse dependents in topological order, and write names of changed
        // packages to standard output if they have a version increment
        let dependents = context.workspace.dependents()?;
        for node in &dependents {
            if increments[node].is_some() {
                let name = dependents[node].name().expect("invariant");
                println!("{name}");
            }
        }

        // No errors occurred
        Ok(())
    }
}
