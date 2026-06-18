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

//! Cargo manifest.

use semver::Version;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;

use crate::project::manifest::{Manifest, ManifestFile};
use crate::project::workspace::Versions;
use crate::project::{Error, Result};

mod model;
mod versions;

use model::{Dependency, Package, Workspace};

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Cargo manifest.
///
/// Note that we only read parts of the manifest relevant to our use case, as
/// we're solely interested in identifying package name, version, and workspace
/// members, and dependencies, in order to bump versions. Other fields can be
/// safely ignored, so we don't model them here.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Cargo {
    /// Cargo workspace.
    Workspace {
        /// Workspace data.
        workspace: Workspace,
    },
    /// Cargo package.
    Package {
        /// Package data.
        package: Package,
        /// Package dependencies.
        #[serde(default)]
        dependencies: BTreeMap<String, Dependency>,
    },
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Manifest for Cargo {
    /// Returns a reference to the name.
    #[inline]
    fn name(&self) -> Option<&str> {
        if let Cargo::Package { package, .. } = self {
            Some(&package.name)
        } else {
            None
        }
    }

    /// Returns a reference to the version.
    #[inline]
    fn version(&self) -> Option<&Version> {
        if let Cargo::Package { package, .. } = self {
            Some(&package.version)
        } else {
            None
        }
    }

    /// Returns a reference to the members.
    #[inline]
    fn members(&self) -> Cow<'_, [String]> {
        if let Cargo::Workspace { workspace } = self {
            Cow::Borrowed(&workspace.members)
        } else {
            Cow::Borrowed(&[])
        }
    }

    /// Creates an iterator over the dependencies.
    #[inline]
    fn dependencies(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        let dependencies = match self {
            Cargo::Package { dependencies, .. } => dependencies,
            Cargo::Workspace { workspace } => &workspace.dependencies,
        };

        // Return iterator over dependency names
        Box::new(dependencies.keys().map(String::as_str))
    }

    /// Updates the given manifest's content with new package versions.
    #[inline]
    fn update(&self, content: &str, versions: &Versions<'_>) -> Result<String> {
        versions::update(content, versions)
    }

    /// Synchronizes the manifest after update.
    ///
    /// Note that this method will run `cargo update --workspace --offline` to
    /// synchronize `Cargo.lock` with the updated versions. This is necessary
    /// to ensure that the lock file reflects the changes made to the manifest,
    /// or there will be inconsistencies between the manifest and the lock file.
    fn sync(&self, path: &Path) -> Result {
        // Explicitly update `Cargo.lock` for synchronization
        let status = Command::new("cargo")
            .args(["update", "--workspace", "--offline"])
            .current_dir(path)
            .stderr(Stdio::null())
            .status()?;

        // Return error with status code if unsuccessful
        status.success().then_some(()).ok_or(Error::Status(status))
    }
}

impl ManifestFile for Cargo {
    /// Manifest file name.
    const FILE: &'static str = "Cargo.toml";
}

// ----------------------------------------------------------------------------

impl FromStr for Cargo {
    type Err = Error;

    /// Attempts to create a manifest from a string.
    #[inline]
    fn from_str(value: &str) -> Result<Self> {
        Ok(toml::from_str(value)?)
    }
}
