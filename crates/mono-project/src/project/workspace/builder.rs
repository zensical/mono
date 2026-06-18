// Copyright (c) 2025-2026 Zensical and contributors

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

//! Workspace builder.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::project::manifest::Manifest;
use crate::project::Project;

use super::error::{Error, Result};
use super::Workspace;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Workspace builder.
#[derive(Debug)]
pub struct Builder<T>
where
    T: Manifest,
{
    /// Workspace path.
    path: PathBuf,
    /// Workspace projects.
    projects: BTreeMap<PathBuf, Project<T>>,
    /// Workspace packages.
    packages: BTreeMap<String, PathBuf>,
    /// Workspace scopes.
    scopes: BTreeMap<String, PathBuf>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<T> Workspace<T>
where
    T: Manifest,
{
    /// Creates a workspace builder.
    #[inline]
    #[must_use]
    pub fn builder<P>(path: P) -> Builder<T>
    where
        T: Manifest,
        P: AsRef<Path>,
    {
        Builder {
            path: path.as_ref().to_path_buf(),
            projects: BTreeMap::new(),
            packages: BTreeMap::new(),
            scopes: BTreeMap::new(),
        }
    }
}

// ----------------------------------------------------------------------------

impl<T> Builder<T>
where
    T: Manifest,
{
    /// Adds a project to the workspace.
    #[allow(clippy::missing_panics_doc)]
    pub fn add<S>(&mut self, scope: S, project: Project<T>)
    where
        S: Into<String>,
    {
        let root = project.path().parent().expect("invariant").to_path_buf();
        let name = project.name().expect("invariant").to_string();

        // Associate package name and scope with the project root
        self.packages.insert(name, root.clone());
        self.scopes.insert(scope.into(), root);

        // Add project to workspace
        self.insert(project);
    }

    /// Adds an unnamed project to the workspace.
    ///
    /// Cargo uses workspaces to group related packages together, and packages
    /// have their own names. However, the workspace doesn't have a name.
    #[allow(clippy::missing_panics_doc)]
    pub(super) fn insert(&mut self, project: Project<T>) {
        let root = project.path().parent().expect("invariant").to_path_buf();
        self.projects.insert(root, project);
    }

    /// Builds the workspace.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Empty`] if the workspace has no projects.
    #[inline]
    pub fn build(self) -> Result<Workspace<T>> {
        if self.packages.is_empty() {
            return Err(Error::Empty);
        }

        // Return workspace
        Ok(Workspace {
            path: self.path,
            projects: self.projects,
            packages: self.packages,
            scopes: self.scopes,
        })
    }
}
