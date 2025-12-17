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

//! Project.

use semver::Version;
use std::fmt::Write;
use std::iter::{Chain, Once};
use std::path::{Path, PathBuf};
use std::{fmt, fs, iter};

mod error;
pub mod manifest;
mod members;
pub mod version;
pub mod workspace;

pub use error::{Error, Result};
use manifest::Manifest;
use members::Members;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Project.
#[derive(Debug)]
pub struct Project<T>
where
    T: Manifest,
{
    /// Project path.
    path: PathBuf,
    /// Project manifest.
    manifest: T,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<T> Project<T>
where
    T: Manifest,
{
    /// Attempts to read a project from the given path.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Io`], if the project could not be read.
    pub fn read<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        Ok(Self {
            path: path.canonicalize()?,
            manifest: content.parse()?,
        })
    }

    /// Returns a reference to the path.
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns a reference to the name.
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.manifest.name()
    }

    /// Returns a reference to the version.
    #[inline]
    pub fn version(&self) -> Option<&Version> {
        self.manifest.version()
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<T> PartialEq for Project<T>
where
    T: Manifest,
{
    /// Compares two projects for equality.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl<T> Eq for Project<T> where T: Manifest {}

// ----------------------------------------------------------------------------

impl<T> IntoIterator for Project<T>
where
    T: Manifest,
{
    type Item = Result<Project<T>>;
    type IntoIter = Chain<Once<Self::Item>, Members<T>>;

    /// Creates an iterator over the project and its members.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let members = self.members();
        iter::once(Ok(self)).chain(members)
    }
}

// ----------------------------------------------------------------------------

impl<T> fmt::Display for Project<T>
where
    T: Manifest,
{
    /// Formats the project for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (Some(name), Some(version)) = (self.name(), self.version()) else {
            return f.write_str("(workspace)");
        };

        // Write name and version
        name.fmt(f)?;
        f.write_char('@')?;
        version.fmt(f)
    }
}
