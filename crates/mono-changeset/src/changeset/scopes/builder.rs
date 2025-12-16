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

//! Scope set builder.

use globset::{Glob, GlobSetBuilder};
use std::path::{Path, PathBuf};

use super::error::{Error, Result};
use super::Scopes;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Scope set builder.
#[derive(Debug)]
pub struct Builder {
    /// Registered scopes.
    paths: Vec<(PathBuf, String)>,
    /// Glob set builder.
    globs: GlobSetBuilder,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Builder {
    /// Creates a scope set builder.
    ///
    /// Note that the canonical way to create [`Scopes`] is to invoke the
    /// [`Scopes::builder`] method, which creates an instance of [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use mono_changeset::scopes::Builder;
    ///
    /// // Create scope set builder
    /// let mut builder = Builder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            globs: GlobSetBuilder::new(),
        }
    }

    /// Adds a scope to the scope set.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Glob`] if the [`Glob`] can't be built.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mono_changeset::Scopes;
    ///
    /// // Create scope set builder and add scope
    /// let mut builder = Scopes::builder();
    /// builder.add("crates/mono", "mono")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn add<P, N>(&mut self, path: P, name: N) -> Result<&mut Self>
    where
        P: AsRef<Path>,
        N: Into<String>,
    {
        let path = path.as_ref();
        if !path.is_relative() {
            return Err(Error::PathAbsolute);
        }

        // Ensure path does not already exist, as scopes must be unique
        if self.paths.iter().any(|(candidate, _)| candidate == path) {
            Err(Error::PathExists)

        // Create pattern matching all files under the given path
        } else {
            let glob = path.join("**");

            // Create glob and add to builder
            self.paths.push((path.to_path_buf(), name.into()));
            self.globs.add(Glob::new(&glob.to_string_lossy())?);

            // Return builder for chaining
            Ok(self)
        }
    }

    /// Builds the scope set.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Glob`] if the [`GlobSet`][] can't be built.
    ///
    /// [`GlobSet`]: globset::GlobSet
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mono_changeset::Scopes;
    ///
    /// // Create scope set builder and add scope
    /// let mut builder = Scopes::builder();
    /// builder.add("crates/mono", "mono")?;
    ///
    /// // Create scope set from builder
    /// let scopes = builder.build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> Result<Scopes> {
        Ok(Scopes {
            paths: self.paths.into_iter().collect(),
            globs: self.globs.build()?,
        })
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Default for Builder {
    /// Creates a scope set builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use mono_changeset::scopes::Builder;
    ///
    /// // Create scope set builder
    /// let mut builder = Builder::default();
    /// ```
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
