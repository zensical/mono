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

//! Changeset.

use mono_project::version::Increment;
use mono_project::{Manifest, Workspace};
use mono_repository::commit::trim_trailers;

pub mod change;
pub mod changelog;
pub mod config;
mod error;
pub mod revision;
pub mod scopes;

use change::Change;
pub use error::{Error, Result};
use revision::Revision;
use scopes::Scopes;

use crate::changeset::config::Config;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Changeset.
///
/// Changesets extract information from commits, and associate them with a given
/// set of scopes. For all [`Scopes`], an [`Increment`] is derived from changes
/// contained in the commits. This does not include transitive dependencies,
/// which are handled outside of changesets. Changesets only describe.
#[derive(Debug)]
pub struct Changeset<'a> {
    /// Scope set.
    scopes: Scopes,
    /// List of revisions.
    revisions: Vec<Revision<'a>>,
    /// Version increments.
    increments: Vec<Option<Increment>>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Changeset<'_> {
    /// Creates a changeset.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Scopes`] if the scope set can't be built
    /// from the workspace, which should practically never happen.
    pub fn new<T>(workspace: &Workspace<T>) -> Result<Self>
    where
        T: Manifest,
    {
        Self::with_config(workspace, &Config::default())
    }

    /// Creates a changeset with the given configuration.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Scopes`] if the scope set can't be built
    /// from the workspace and configuration, most commonly due to path issues.
    pub fn with_config<T>(
        workspace: &Workspace<T>, config: &Config,
    ) -> Result<Self>
    where
        T: Manifest,
    {
        let mut builder = Scopes::builder();
        for (path, name) in workspace.packages() {
            builder.add(path, name)?;
        }

        // Append additional scopes from configuration
        for (name, path) in &config.scopes {
            builder.add(path, name)?;
        }

        // Create scope set and version increments
        let scopes = builder.build()?;
        Ok(Self {
            increments: vec![None; scopes.len()],
            scopes,
            revisions: Vec::new(),
        })
    }

    /// Returns the summary.
    ///
    /// The summary is given by the commit's body of the latest revision in the
    /// changeset, trimming off any trailers like `Signed-off-by`.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Summary`] if no summary can be extracted,
    /// so either there are no revisions, or the latest commit has no body. We
    /// deliberately turn this into an error to ensure that the release process
    /// can always rely on a summary being present.
    pub fn summary(&self) -> Result<&str> {
        let opt = self.revisions.first();
        let summary = opt
            .and_then(|revision| revision.commit().body())
            .ok_or(Error::Summary)?;

        // Trim trailers and ensure non-empty summary
        let summary = trim_trailers(summary)?.trim();
        (!summary.is_empty())
            .then_some(summary)
            .ok_or(Error::Summary)
    }
}

#[allow(clippy::must_use_candidate)]
impl Changeset<'_> {
    /// Returns a reference to the scope set.
    #[inline]
    pub fn scopes(&self) -> &Scopes {
        &self.scopes
    }

    /// Returns a reference to the list of revisions.
    #[inline]
    pub fn revisions(&self) -> &[Revision<'_>] {
        &self.revisions
    }

    /// Returns a reference to the version increments.
    #[inline]
    pub fn increments(&self) -> &[Option<Increment>] {
        &self.increments
    }

    /// Returns the number of revisions.
    #[inline]
    pub fn len(&self) -> usize {
        self.revisions.len()
    }

    /// Returns whether there are any revisions.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.revisions.is_empty()
    }
}
