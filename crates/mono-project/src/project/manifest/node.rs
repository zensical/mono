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

//! Node manifest.

use semver::{Version, VersionReq};
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;

use crate::project::manifest::{Manifest, ManifestFile};
use crate::project::workspace::Versions;
use crate::project::{Error, Result};

mod versions;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Node manifest.
///
/// Note that we only read parts of the manifest relevant to our use case, as
/// we're solely interested in identifying package name, version, and workspace
/// members, and dependencies, in order to bump versions. Other fields can be
/// safely ignored, so we don't model them here.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// Package name.
    pub name: String,
    /// Package version.
    pub version: Version,
    /// Package workspace members.
    #[serde(default)]
    pub workspaces: Vec<String>,
    /// Package dependencies.
    #[serde(default)]
    pub dependencies: BTreeMap<String, VersionReq>,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Manifest for Node {
    /// Returns a reference to the name.
    #[inline]
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    /// Returns a reference to the version.
    #[inline]
    fn version(&self) -> Option<&Version> {
        Some(&self.version)
    }

    /// Returns a reference to the members.
    #[inline]
    fn members(&self) -> Cow<'_, [String]> {
        Cow::Borrowed(&self.workspaces)
    }

    /// Creates an iterator over the dependencies.
    #[inline]
    fn dependencies(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(self.dependencies.keys().map(String::as_str))
    }

    /// Updates the given manifest's content with new package versions.
    #[inline]
    fn update(&self, content: &str, versions: &Versions<'_>) -> Result<String> {
        versions::update(content, versions)
    }

    /// Synchronizes the manifest after update.
    ///
    /// Note that this method will run `npm install --package-lock-only` to
    /// synchronize `package-lock.json` with the updated versions. This ensures
    /// that the lock file reflects the changes made to the manifest, or there
    /// will be inconsistencies between the manifest and the lock file.
    fn sync(&self, path: &Path) -> Result {
        // Explicitly update `package-lock.json` for synchronization
        let status = Command::new("npm")
            .args(["install", "--package-lock-only", "--ignore-scripts"])
            .current_dir(path)
            .stderr(Stdio::null())
            .status()?;

        // Return error with status code if unsuccessful
        status.success().then_some(()).ok_or(Error::Status(status))
    }
}

impl ManifestFile for Node {
    /// Manifest file name.
    const FILE: &'static str = "package.json";
}

// ----------------------------------------------------------------------------

impl FromStr for Node {
    type Err = Error;

    /// Attempts to create a manifest from a string.
    #[inline]
    fn from_str(value: &str) -> Result<Self> {
        Ok(serde_json::from_str(value)?)
    }
}
