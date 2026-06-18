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

//! Manifest.

use semver::Version;
use std::borrow::Cow;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

use super::error::{Error, Result};
use super::workspace::Versions;

pub mod cargo;
pub mod node;

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

/// Manifest.
///
/// Manifests can be packages or workspaces – sometimes one or the other, and
/// sometimes both at the same time, depending on the ecosystem. This is also
/// why several methods of this trait return optional references – ecosystems
/// differ in how they implement these concepts (e.g. Cargo and Node).
///
/// Note that manifests only return the names of their dependencies, not their
/// version requirements, since we only require inner-workspace dependencies,
/// which we resolve as part of workspace resolution. Also, some ecosystems
/// like Rust support inheriting version requirements from the workspace.
///
/// Think of this trait as being an adapter into an ecosystem-specific manifest
/// format, providing just enough information for version management.
pub trait Manifest: Debug {
    /// Returns a reference to the name.
    fn name(&self) -> Option<&str>;

    /// Returns a reference to the version.
    fn version(&self) -> Option<&Version>;

    /// Returns a reference to the members.
    fn members(&self) -> Cow<'_, [String]>;

    /// Creates an iterator over the dependencies.
    fn dependencies(&self) -> Box<dyn Iterator<Item = &str> + '_>;

    /// Updates the given manifest's content with new package versions.
    ///
    /// # Errors
    ///
    /// This method must return an error if the content cannot be transformed,
    /// e.g. when it can't be successfully parsed, verified, or serialized.
    fn update(&self, content: &str, versions: &Versions<'_>) -> Result<String>;

    /// Synchronizes the manifest after update.
    ///
    /// # Errors
    ///
    /// This method must return an error if synchronization fails.
    fn sync(&self, path: &Path) -> Result;
}

/// Manifest file.
pub trait ManifestFile: Manifest + FromStr<Err = Error> {
    /// Manifest file name.
    const FILE: &'static str;
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<T> Manifest for Box<T>
where
    T: Manifest + ?Sized,
{
    /// Returns a reference to the name.
    #[inline]
    fn name(&self) -> Option<&str> {
        (**self).name()
    }

    /// Returns a reference to the version.
    #[inline]
    fn version(&self) -> Option<&Version> {
        (**self).version()
    }

    /// Returns a reference to the members.
    #[inline]
    fn members(&self) -> Cow<'_, [String]> {
        (**self).members()
    }

    /// Creates an iterator over the dependencies.
    #[inline]
    fn dependencies(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        (**self).dependencies()
    }

    /// Updates the given manifest's content with new package versions.
    #[inline]
    fn update(&self, content: &str, versions: &Versions<'_>) -> Result<String> {
        (**self).update(content, versions)
    }

    /// Synchronizes the manifest after update.
    #[inline]
    fn sync(&self, path: &Path) -> Result {
        (**self).sync(path)
    }
}
