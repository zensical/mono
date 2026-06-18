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
        let name = project.name().expect("invariant");

        // Associate package name with project root and add to workspace
        self.packages.insert(name.to_string(), root.clone());
        self.projects.insert(root.clone(), project);

        // Associate scope with project root
        self.scopes.insert(scope.into(), root);
    }

    /// Builds the workspace.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Empty`] if the workspace has no projects.
    #[inline]
    pub fn build(self) -> Result<Workspace<T>> {
        if self.projects.is_empty() {
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
