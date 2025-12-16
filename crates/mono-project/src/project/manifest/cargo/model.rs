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

//! Cargo manifest model.

use semver::{Version, VersionReq};
use serde::Deserialize;
use std::collections::BTreeMap;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Cargo dependency.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    /// Dependency with version requirement.
    Version(VersionReq),
    /// Dependency with information.
    Info {
        /// Version requirement.
        version: Option<VersionReq>,
    },
}

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Cargo workspace.
#[derive(Debug, Deserialize)]
pub struct Workspace {
    /// Workspace members.
    pub members: Vec<String>,
    /// Workspace dependencies.
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
}

/// Cargo package.
#[derive(Debug, Deserialize)]
pub struct Package {
    /// Package name.
    pub name: String,
    /// Package version.
    pub version: Version,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Dependency {
    /// Returns the version requirement.
    #[must_use]
    pub fn version(&self) -> Option<&VersionReq> {
        match self {
            Dependency::Version(version) => Some(version),
            Dependency::Info { version } => version.as_ref(),
        }
    }
}
