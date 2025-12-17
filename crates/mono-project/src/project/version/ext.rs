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

//! Version extensions.

use semver::{BuildMetadata, Error, Prerelease, Version};

use super::increment::Increment;

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

/// Extension of [`Version`].
pub trait VersionExt {
    /// Returns the next version after applying the given increment.
    fn bump(&self, increment: Increment) -> Version;

    /// Returns the minimum increment for the version.
    fn min_bump(&self) -> Option<Increment>;

    /// Returns the maximum increment for the version.
    fn max_bump(&self) -> Increment;

    /// Parses a version from a string, allowing for an optional `v` prefix.
    ///
    /// Internally, versions are represented without the `v` prefix, but when
    /// parsing from user input, it's common to include it, since all git tags
    /// are prefixed with `v` for easier discoverability and discernability.
    /// For this reason, the CLI encourages the use of the `v` prefix.
    ///
    /// # Errors
    ///
    /// This method returns [`Error`] if parsing fails.
    fn from_str_with_prefix(value: &str) -> Result<Version, Error> {
        value.trim_start_matches('v').parse()
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl VersionExt for Version {
    /// Returns the next version after applying the given increment.
    ///
    /// This method returns the next version after applying the given increment,
    /// strictly following semantic versioning rules.
    ///
    /// Versions in the `0.y.z` and `0.0.z` range receive special handling, in
    /// that `0.y.z` versions only increment the minor version for minor and
    /// major increments, while `0.0.z` versions always increment the patch
    /// version. In both cases, moving versioning one digit to the left is not
    /// supported, so it's impossible to accidentally turn `0.0.z` into `0.1.0`.
    /// If that is desired, an explicit version bump is required.
    fn bump(&self, increment: Increment) -> Version {
        let mut version = self.clone();

        // Apply increment according to semantic versioning rules
        match (self.major, self.minor, increment) {
            // 0.0.z -> 0.0.z+1
            (0, 0, _) => {
                version.patch = version.patch.saturating_add(1);
            }
            // 0.y.z -> 0.y+1.0
            (0, _, Increment::Major | Increment::Minor) => {
                version.minor = version.minor.saturating_add(1);
                version.patch = 0;
            }
            // 0.y.z -> 0.y.z+1
            (0, _, Increment::Patch) => {
                version.patch = version.patch.saturating_add(1);
            }
            // x.y.z -> x+1.0.0
            (_, _, Increment::Major) => {
                version.major = version.major.saturating_add(1);
                version.minor = 0;
                version.patch = 0;
            }
            // x.y.z -> x.y+1.0
            (_, _, Increment::Minor) => {
                version.minor = version.minor.saturating_add(1);
                version.patch = 0;
            }
            // x.y.z -> x.y.z+1
            (_, _, Increment::Patch) => {
                version.patch = version.patch.saturating_add(1);
            }
        }

        // Always reset pre-release identifier and build metadata
        version.pre = Prerelease::EMPTY;
        version.build = BuildMetadata::EMPTY;
        version
    }

    /// Returns the minimum increment for the version.
    fn min_bump(&self) -> Option<Increment> {
        if let (0, 0) = (self.major, self.minor) {
            Some(Increment::Patch)
        } else {
            None
        }
    }

    /// Returns the maximum increment for the version.
    fn max_bump(&self) -> Increment {
        match (self.major, self.minor) {
            (0, 0) => Increment::Patch,
            (0, _) => Increment::Minor,
            (_, _) => Increment::Major,
        }
    }
}
