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

//! Iterator over members of a project.

use std::marker::PhantomData;

use super::error::Result;
use super::manifest::Manifest;
use super::Project;

mod paths;

use paths::Paths;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Iterator over members of a project.
///
/// This iterator emits projects recursively. Although some ecosystems don't
/// allow for deeply nested project hierachies, it's possible to have them.
#[derive(Debug)]
pub struct Members<T> {
    /// Stack of path iterators.
    paths: Vec<Paths>,
    /// Manifest file name.
    file: String,
    /// Type marker.
    marker: PhantomData<T>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<T> Project<T>
where
    T: Manifest,
{
    /// Creates an iterator over the members of the project.
    ///
    /// This iterator only emits members, not the root project itself. In case
    /// you want to include the root project, iterate over [`Project`] itself.
    #[allow(clippy::missing_panics_doc)]
    pub fn members(&self) -> Members<T> {
        let root = self.path.parent().expect("invariant");
        let file = self.path.file_name().expect("invariant");

        // Create path iterator over members and initialize stack
        let data = self.manifest.members();
        let iter = data.iter().map(|path| root.join(path));
        Members {
            paths: vec![iter.rev().collect()],
            file: file.to_string_lossy().to_string(),
            marker: PhantomData,
        }
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<T> Iterator for Members<T>
where
    T: Manifest,
{
    type Item = Result<Project<T>>;

    /// Returns the next member.
    fn next(&mut self) -> Option<Self::Item> {
        // Retrieve the top-most path iterator from the stack and try to draw
        // another path from it, or if empty, continue with the next iterator
        let stack = self.paths.last_mut()?;
        let Some(res) = stack.next() else {
            self.paths.pop();
            return self.next();
        };

        // Read project from path after joining with the name of the manifest
        // file, and if successful, push nested paths iterator onto the stack
        match res
            .map(|path| path.join(&self.file))
            .and_then(Project::read)
        {
            Err(err) => Some(Err(err)),
            Ok(project) => {
                let members = project.members();
                self.paths.extend(members.paths);

                // Return next member
                Some(Ok(project))
            }
        }
    }
}
