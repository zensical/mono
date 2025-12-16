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

use std::collections::btree_map::{Values, ValuesMut};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::error::Result;
use super::manifest::Manifest;
use super::Project;

mod dependents;
mod packages;
mod versions;

pub use dependents::Dependents;
pub use packages::Packages;
pub use versions::Versions;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Workspace.
#[derive(Debug)]
pub struct Workspace<T>
where
    T: Manifest,
{
    /// Workspace path.
    path: PathBuf,
    /// Workspace projects.
    projects: BTreeMap<PathBuf, Project<T>>,
    /// Workspace packages.
    packages: BTreeMap<String, PathBuf>,
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
    /// This method returns [`Error::Io`][], if the workspace could not be read.
    ///
    /// [`Error::Io`]: crate::project::Error::Io
    #[allow(clippy::missing_panics_doc)]
    pub fn read<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let project = Project::<T>::read(path.as_ref())?;

        // Extract root path of workspace, so we can make paths relative when
        // necessary (e.g. for scopes), and create iterator over projects
        let root = project.path.parent().expect("invariant").to_path_buf();
        let iter = project.into_iter().map(|res| {
            res.map(|package| {
                let base = package.path.parent().expect("invariant");
                (base.to_path_buf(), package)
            })
        });

        // Collect projects and extract packages, so we can map package names
        // to their paths in order to resolve projects by package name
        let projects = iter.collect::<Result<BTreeMap<_, _>>>()?;
        let iter = projects.iter().filter_map(|(path, project)| {
            let opt = project.manifest.name();
            opt.map(|name| (name.to_string(), path.clone()))
        });

        // Collect packages and return workspace
        let packages = iter.collect::<BTreeMap<_, _>>();
        Ok(Self { path: root, projects, packages })
    }

    /// Attempts to resolve a workspace at the given path.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Io`][], if the workspace could not be read.
    ///
    /// [`Error::Io`]: crate::project::Error::Io
    #[inline]
    pub fn resolve<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Self::read(T::resolve(path.as_ref())?)
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

    /// Creates a mutable iterator over the workspace.
    #[inline]
    pub fn iter_mut(&mut self) -> ValuesMut<'_, PathBuf, Project<T>> {
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

impl<'a, T> IntoIterator for &'a mut Workspace<T>
where
    T: Manifest,
{
    type Item = &'a mut Project<T>;
    type IntoIter = ValuesMut<'a, PathBuf, Project<T>>;

    /// Creates a mutable iterator over the workspace.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.projects.values_mut()
    }
}
