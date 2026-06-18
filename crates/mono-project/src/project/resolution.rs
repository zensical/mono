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

//! Project resolution.

use std::path::Path;

use super::error::Error;
use super::manifest::cargo::Cargo;
use super::manifest::node::Node;
use super::manifest::{Manifest, ManifestFile};
use super::{Project, Result};

// ----------------------------------------------------------------------------
// Type aliases
// ----------------------------------------------------------------------------

/// Project reader function.
type Reader = fn(&Path) -> Result<Project<Box<dyn Manifest>>>;

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Resolves a project manifest from the given path.
pub fn read(path: &Path) -> Result<Project<Box<dyn Manifest>>> {
    let readers = [
        (Cargo::FILE, read_typed::<Cargo> as Reader),
        (Node::FILE, read_typed::<Node> as Reader),
    ];

    // Filter readers for which the manifest file exists in the given path and
    // collect them into a vector of matches
    let matches = readers
        .into_iter()
        .filter(|(file, _)| path.join(file).exists())
        .collect::<Vec<_>>();

    // Read the project using the matching reader, or return an error if no
    // matches were found or if multiple matches were found
    match matches.as_slice() {
        [(_, read)] => read(path),
        [] => Err(Error::NotFound),
        _ => Err(Error::Ambiguous),
    }
}

/// Reads a project manifest from the given path.
fn read_typed<T>(path: &Path) -> Result<Project<Box<dyn Manifest>>>
where
    T: ManifestFile + 'static,
{
    Project::<T>::read(path.join(T::FILE)).map(|project| Project {
        path: project.path,
        manifest: Box::new(project.manifest) as Box<_>,
    })
}
