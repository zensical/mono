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

//! Iterator over commits in a repository.

use std::ops::{Bound, RangeBounds};

use crate::repository::id::Id;
use crate::repository::{Error, Repository, Result};

use super::Commit;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Iterator over commits in a repository.
pub struct Commits<'a> {
    /// Repository.
    repository: &'a Repository,
    /// Git revision walk.
    revwalk: git2::Revwalk<'a>,
    /// End of range.
    end: Option<Id>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Repository {
    /// Creates an iterator over the commits in the repository.
    ///
    /// This method accepts a range of commit identifiers for iteration. If no
    /// bounds are given, iteration ranges from `HEAD` until the end.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`] if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mono_repository::Repository;
    ///
    /// // Open repository
    /// let repo = Repository::open(".")?;
    ///
    /// // Create iterator over commits
    /// for commit in repo.commits(..)?.flatten() {
    ///     println!("{:?}", commit.id());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn commits<R>(&self, range: R) -> Result<Commits<'_>>
    where
        R: RangeBounds<Id>,
    {
        // Create a topological walk over all revisions starting from a given
        // commit, backwards, for as long as the iterator is consumed
        let mut revwalk = self.inner.revwalk()?;
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

        // Determine start and end of range - note that the range is exclusive
        // by default, allowing to easily determine commits between two tags
        let end = match (range.start_bound(), range.end_bound()) {
            // .. - all commits from HEAD
            (Bound::Unbounded, Bound::Unbounded) => {
                revwalk.push_head()?;
                None
            }
            // ..end - commits until end (excluded)
            (Bound::Unbounded, Bound::Excluded(end)) => {
                revwalk.push_head()?;
                Some(*end)
            }
            // start.. - commits from start onwards
            (Bound::Included(start), Bound::Unbounded) => {
                revwalk.push(**start)?;
                None
            }
            // start..end - commits between start and end
            (Bound::Included(start), Bound::Excluded(end)) => {
                revwalk.push(**start)?;
                Some(*end)
            }
            // Unsupported range bound
            (Bound::Excluded(_), _) | (_, Bound::Included(_)) => {
                return Err(Error::Bound);
            }
        };

        // Return iterator over commits
        Ok(Commits { repository: self, revwalk, end })
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<'a> Iterator for Commits<'a> {
    type Item = Result<Commit<'a>>;

    /// Returns the next commit.
    fn next(&mut self) -> Option<Self::Item> {
        let id = match self.revwalk.next()? {
            Ok(id) => id,
            Err(err) => return Some(Err(err.into())),
        };

        // Return next commit, if we haven't reached the end of the range
        if self.end.as_deref() == Some(&id) {
            None
        } else {
            Some(self.repository.get(id))
        }
    }
}
