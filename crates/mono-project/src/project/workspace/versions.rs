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

//! Workspace version set.

use semver::Version;
use std::collections::BTreeMap;
use std::fs;
use std::marker::PhantomData;

use crate::project::manifest::Manifest;
use crate::project::version::{Increment, VersionExt};
use crate::project::Result;

use super::Workspace;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Workspace version set.
///
/// This data type is used to represent a set of package versions in a workspace
/// and only used internally by the [`Workspace::bump`] method. Implementors of
/// [`Manifest`] must also implement [`Writer`][] to support version updates,
/// which must invoke a method on this type to retrieve the new versions.
///
/// [`Writer`]: crate::project::manifest::Writer
pub struct Versions<'a, T> {
    /// Package versions.
    items: BTreeMap<&'a str, Version>,
    /// Type marker.
    marker: PhantomData<T>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<T> Workspace<T>
where
    T: Manifest,
{
    /// Applies the given version increments to all packages in the workspace.
    ///
    /// Note that this method consumes the workspace, since it doesn't update
    /// its internal stte, but needs to be recreated.
    ///
    /// # Errors
    ///
    /// This method returns [`Error:Io`][] if a manifest can't be written to
    /// disk, or any error as encountered by the [`Writer`][] implementation.
    ///
    /// [`Error:Io`]: crate::project::Error::Io
    /// [`Writer`]: crate::project::manifest::Writer
    #[allow(clippy::missing_panics_doc)]
    pub fn bump(self, increments: &[Option<Increment>]) -> Result {
        let mut items = BTreeMap::new();

        // Compute new versions for all packages in workspace, and collect them
        // into a version set that we then pass to each manifest writer
        for (index, (name, path)) in self.packages.iter().enumerate() {
            if let Some(increment) = increments[index] {
                let project = self.projects.get(path).expect("invariant");
                let version = project.version().expect("invariant");
                items.insert(name.as_str(), version.bump(increment));
            }
        }

        // The type marker is only necessary to discern between implementations
        // of the trait for different manifest types
        let versions = Versions { items, marker: PhantomData };
        for project in &self {
            let content = fs::read_to_string(project.path())?;
            fs::write(project.path(), T::update(content, &versions)?)?;
        }

        // Synchronize workspace manifest after update
        T::sync(self.path)
    }
}

// ----------------------------------------------------------------------------

impl<T> Versions<'_, T> {
    /// Returns the version for the given package name.
    #[inline]
    #[must_use]
    pub fn get<N>(&self, name: N) -> Option<&Version>
    where
        N: AsRef<str>,
    {
        self.items.get(name.as_ref())
    }
}
