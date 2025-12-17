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

//! Revision.

use std::cmp;
use std::collections::BTreeSet;
use std::str::FromStr;

use mono_repository::Commit;

use super::change::Change;
use super::error::Result;
use super::Changeset;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Revision.
#[derive(Debug)]
pub struct Revision<'a> {
    /// Original commit.
    commit: Commit<'a>,
    /// Computed change.
    change: Change,
    /// Affected scopes.
    scopes: Vec<usize>,
    /// Relevant issues.
    issues: Vec<u32>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

#[allow(clippy::must_use_candidate)]
impl Revision<'_> {
    /// Returns a reference to the original commit.
    #[inline]
    pub fn commit(&self) -> &Commit<'_> {
        &self.commit
    }

    /// Returns a reference to the computed change.
    #[inline]
    pub fn change(&self) -> &Change {
        &self.change
    }

    /// Returns a reference to the affected scope indices.
    #[inline]
    pub fn scopes(&self) -> &[usize] {
        &self.scopes
    }

    /// Returns a reference to the relevant issues.
    #[inline]
    pub fn issues(&self) -> &[u32] {
        &self.issues
    }
}

// ----------------------------------------------------------------------------

impl<'a> Changeset<'a> {
    /// Adds a commit to the changeset.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Repository`][] if the commit deltas can't
    /// be retrieved. If the commit summary couldn't be parsed, the commit will
    /// be ignored, since there are several types of commits that will not make
    /// it into the changeset, e.g., merge commits.
    ///
    /// [`Error::Repository`]: crate::changeset::Error::Repository
    #[allow(clippy::missing_panics_doc)]
    pub fn add(&mut self, commit: Commit<'a>) -> Result {
        if let Ok(change) = Change::from_str(commit.summary()) {
            // Retrieve affected scopes from commit
            let mut scopes = BTreeSet::new();
            for delta in commit.deltas()? {
                scopes.extend(self.scopes.get(delta.path()));
            }

            // Update increments for affected scopes
            let increment = change.as_increment();
            for &index in &scopes {
                self.increments[index] =
                    cmp::max(self.increments[index], increment);
            }

            // Next, try to find issue references in the commit body, denoted
            // by a hash sign followed by a number, e.g., "#123"
            let issues = commit.body().map(parse_issues).unwrap_or_default();

            // Create revision and add to changeset
            self.revisions.push(Revision {
                commit,
                change,
                scopes: scopes.into_iter().collect(),
                issues: issues.into_iter().collect(),
            });
        }

        // No errors occurred
        Ok(())
    }

    /// Extends the changeset with the given commits.
    ///
    /// Note that we can't just implement the [`Extend`] trait, as this method
    /// is fallible due to potential errors when adding commits.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Repository`][] if a commit's deltas can't
    /// be retrieved. If a commit's message couldn't be parsed, it will just be
    /// ignored, since there are several types of commits that will not make it
    /// into the changset, e.g., merge commits.
    ///
    /// [`Error::Repository`]: crate::changeset::Error::Repository
    pub fn extend<T>(&mut self, iter: T) -> Result
    where
        T: IntoIterator<Item = Commit<'a>>,
    {
        for commit in iter {
            self.add(commit)?;
        }

        // No errors occurred
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Parses issue references from the given commit body, e.g., `#123`.
fn parse_issues(body: &str) -> BTreeSet<u32> {
    let iter = body.split_whitespace().filter_map(|word| {
        word.trim_matches(|char: char| !char.is_ascii_digit() && char != '#')
            .strip_prefix('#')
            .and_then(|num| num.parse().ok())
    });

    // Collect issue references into set to avoid duplicates
    iter.collect()
}
