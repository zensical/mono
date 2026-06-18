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

//! Workspace.

use std::collections::btree_map::Values;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::manifest::{Manifest, ManifestFile};
use super::Project;

mod builder;
mod dependents;
mod error;
mod scopes;
mod versions;

pub use builder::Builder;
pub use dependents::Dependents;
pub use error::{Error, Result};
pub use scopes::Scopes;
pub use versions::Versions;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Workspace.
#[derive(Debug)]
pub struct Workspace<T = Box<dyn Manifest>>
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
    /// Attempts to read a workspace from the given path.
    ///
    /// This method attempts to read the top-level project at the given path,
    /// and then discovers all member projects defined in the workspace.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Io`][], if the workspace could not be read.
    #[allow(clippy::missing_panics_doc)]
    pub fn read<P>(path: P) -> Result<Self>
    where
        T: ManifestFile,
        P: AsRef<Path>,
    {
        let project = Project::<T>::read(path.as_ref())?;
        let root = project.path.parent().expect("invariant").to_path_buf();

        // Collect all discovered projects into the workspace builder - note
        // that in regular workspaces, scopes and package names are identical
        let mut builder = Self::builder(root);
        for res in project {
            let project = res?;

            // Set project name as scope and add to workspace - if a project
            // does not have a name, skip it as it's most likely a workspace
            if let Some(name) = project.name() {
                builder.add(name.to_string(), project);
            }
        }

        // Return workspace
        builder.build()
    }

    /// Attempts to resolve a workspace at the given path.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Io`][], if the workspace could not be read.
    ///
    /// [`Error::Io`]: crate::project::Error::Io
    #[inline]
    pub fn resolve<P>(path: P) -> Result<Self>
    where
        T: ManifestFile,
        P: AsRef<Path>,
    {
        Self::read(path.as_ref().join(T::FILE))
    }

    /// Returns a reference to the project with the given name.
    #[inline]
    #[must_use]
    pub fn get<N>(&self, name: N) -> Option<&Project<T>>
    where
        N: AsRef<str>,
    {
        self.projects.get(self.packages.get(name.as_ref())?)
    }

    /// Creates an iterator over the workspace.
    #[inline]
    pub fn iter(&self) -> Values<'_, PathBuf, Project<T>> {
        self.into_iter()
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<'a, T> IntoIterator for &'a Workspace<T>
where
    T: Manifest,
{
    type Item = &'a Project<T>;
    type IntoIter = Values<'a, PathBuf, Project<T>>;

    /// Creates an iterator over the workspace.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.projects.values()
    }
}
