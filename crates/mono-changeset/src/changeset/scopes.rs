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

//! Scope set.

use globset::GlobSet;
use std::fmt;
use std::ops::Index;
use std::path::{Path, PathBuf};

mod builder;
mod error;

pub use builder::Builder;
pub use error::{Error, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Scope set.
///
/// Scopes are used to associate changes with non-overlapping paths in a git
/// repository, where a list of paths is matched through a [`GlobSet`]. When
/// two paths overlap, one path must be the prefix of another path. Thus, we
/// return the longer path as the matching scope.
pub struct Scopes {
    /// Registered scopes.
    paths: Vec<(PathBuf, String)>,
    /// Glob set.
    globs: GlobSet,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Scopes {
    /// Creates a scope set builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use mono_changeset::Scopes;
    ///
    /// // Create scope set builder
    /// let mut builder = Scopes::builder();
    #[inline]
    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Returns the longest matching scope for the given path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mono_changeset::Scopes;
    /// use std::path::Path;
    ///
    /// // Create scope set builder and add path
    /// let mut builder = Scopes::builder();
    /// builder.add("crates/mono", "mono")?;
    ///
    /// // Create scope set from builder
    /// let scopes = builder.build()?;
    ///
    /// // Create path and obtain longest matching scope
    /// let path = Path::new("crates/mono/Cargo.toml");
    /// assert_eq!(scopes.get(&path), Some(0));
    /// # Ok(())
    /// # }
    /// ```
    pub fn get<P>(&self, path: P) -> Option<usize>
    where
        P: AsRef<Path>,
    {
        self.globs.matches(path).into_iter().max_by_key(|&index| {
            let (path, _) = &self.paths[index];
            path.components().count()
        })
    }
}

#[allow(clippy::must_use_candidate)]
impl Scopes {
    /// Returns the number of scopes.
    #[inline]
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    /// Returns whether there are any scopes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Index<usize> for Scopes {
    type Output = (PathBuf, String);

    /// Returns the scope at the given index.
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.paths[index]
    }
}

// ----------------------------------------------------------------------------

impl fmt::Debug for Scopes {
    /// Formats the scope set for debugging.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Scope")
            .field("paths", &self.paths)
            .finish_non_exhaustive()
    }
}
