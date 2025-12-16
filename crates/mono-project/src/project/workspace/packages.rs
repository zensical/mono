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

//! Iterator over packages in a workspace.

use std::collections::btree_map::Iter;
use std::path::PathBuf;

use crate::project::manifest::Manifest;

use super::Workspace;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Iterator over packages in a workspace.
///
/// This iterator emits package names and paths relative to the workspace root,
/// so they can be used together with the commit deltas from the repository.
pub struct Packages<'a> {
    /// Workspace path.
    path: &'a PathBuf,
    /// Workspace packages iterator.
    iter: Iter<'a, String, PathBuf>,
}

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

impl<T> Workspace<T>
where
    T: Manifest,
{
    /// Creates an iterator over the packages in the workspace.
    ///
    /// Note that packages are represented as a tuple of path and name, since
    /// this is what is needed to determine the scopes of a commit. In order to
    /// obtain the [`Project`][] for a package by its name, [`Workspace::get`]
    /// can be used.
    ///
    /// [`Project`]: crate::project::Project
    #[inline]
    #[must_use]
    pub fn packages(&self) -> Packages<'_> {
        Packages {
            path: &self.path,
            iter: self.packages.iter(),
        }
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Iterator for Packages<'_> {
    type Item = (PathBuf, String);

    /// Returns the next package.
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(name, path)| {
            let path = path.strip_prefix(self.path).expect("invariant");
            (path.to_path_buf(), name.clone())
        })
    }
}
