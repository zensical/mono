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

//! Version set.

use semver::Version;
use std::collections::btree_map::{Iter, Range};
use std::collections::BTreeMap;
use std::fmt;
use std::iter::Rev;
use std::ops::RangeBounds;

use super::commits::Commits;
use super::error::{Error, Result};
use super::id::Id;
use super::Repository;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Version set.
///
/// This data type manages the existing versions in a given repository. Versions
/// are ordered chronologically, so iteration and range queries are simple. Each
/// version is mapped to the identifier of its corresponding commit, so commits
/// can be obtained to query for changes between two versions.
pub struct Versions<'a> {
    /// Repository.
    repository: &'a Repository,
    /// Versions and their corresponding commit identifiers.
    tags: BTreeMap<Version, Id>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Repository {
    /// Returns the version set of the repository.
    ///
    /// This method only extracts the tags matching semantic version specifiers
    /// from the given repository, and returns a version set. Tags must abide
    /// to the `vMAJOR.MINOR.PATCH` format, but can include pre-release and
    /// build suffixes as well. Each tag is parsed as a [`Version`].
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`] if the operation fails.
    pub fn versions(&self) -> Result<Versions<'_>> {
        let tags = self.inner.tag_names(Some("v[0-9]*.[0-9]*.[0-9]**"))?;
        let iter = tags.iter().flatten().map(|name| {
            let version = name.trim_start_matches('v').parse()?;
            Ok((version, self.find(name)?.id()))
        });

        // Collect and return version set
        let tags = iter.collect::<Result<_>>()?;
        Ok(Versions { repository: self, tags })
    }
}

// ----------------------------------------------------------------------------

impl Versions<'_> {
    /// Returns the commit identifier for the given version.
    #[inline]
    #[must_use]
    pub fn get(&self, version: &Version) -> Option<&Id> {
        self.tags.get(version)
    }

    /// Returns whether the version set contains the given version.
    #[inline]
    #[must_use]
    pub fn contains(&self, version: &Version) -> bool {
        self.tags.contains_key(version)
    }

    /// Creates a range iterator over the version set.
    #[inline]
    pub fn range<R>(&self, range: R) -> Range<'_, Version, Id>
    where
        R: RangeBounds<Version>,
    {
        // we don't range iter over the version set BUT over the commits!
        self.tags.range(range)
    }

    /// Creates an iterator over the commits for the given version.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Version`] if the given version doesn't
    /// exist, or [`Error::Git`] if the operation fails on the repository.
    pub fn commits(&self, version: Option<&Version>) -> Result<Commits<'_>> {
        if let Some(version) = version {
            if !self.tags.contains_key(version) {
                return Err(Error::Version);
            }

            // In case the given version is the first version in the repository,
            // start at the commit tagged with this version and continue until
            // the first commit. Otherwise, stop just before the commit which
            // is tagged with the previous version.
            let mut iter = self.tags.range(..=version).rev();
            if let Some((_, start)) = iter.next() {
                if let Some((_, end)) = iter.next() {
                    self.repository.commits(start..end)
                } else {
                    self.repository.commits(start..)
                }
            } else {
                Err(Error::Version)
            }
        } else {
            // No version given, so return all commits in the repository until
            // we either reach the previous version or the first commit
            let mut iter = self.tags.range(..).rev();
            if let Some((_, end)) = iter.next() {
                self.repository.commits(..end)
            } else {
                self.repository.commits(..)
            }
        }
    }

    /// Creates an iterator over the version set.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&Version, &Id)> {
        self.into_iter()
    }
}

#[allow(clippy::must_use_candidate)]
impl Versions<'_> {
    /// Returns the number of versions.
    #[inline]
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// Returns whether there are any versions.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<'a> IntoIterator for &'a Versions<'a> {
    type Item = (&'a Version, &'a Id);
    type IntoIter = Rev<Iter<'a, Version, Id>>;

    /// Creates an iterator over the version set.
    fn into_iter(self) -> Self::IntoIter {
        self.tags.iter().rev()
    }
}

// ----------------------------------------------------------------------------

impl fmt::Debug for Versions<'_> {
    /// Formats the version set for debugging.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Versions")
            .field("tags", &self.tags)
            .finish()
    }
}
