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

//! Iterator over deltas in a commit.

use crate::repository::commit::Commit;
use crate::repository::Result;

use super::Delta;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Iterator over deltas in a commit.
pub struct Deltas<'a> {
    /// Git diff.
    inner: git2::Diff<'a>,
    /// Current index.
    index: usize,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Commit<'_> {
    /// Creates an iterator over the deltas in the commit.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`][] if the operation fails.
    ///
    /// [`Error::Git`]: crate::repository::Error::Git
    pub fn deltas(&self) -> Result<Deltas<'_>> {
        let tree = self.inner.tree()?;

        // Obtain parent tree, if any
        let parent = if self.inner.parent_count() > 0 {
            Some(self.inner.parent(0)?.tree()?)
        } else {
            None
        };

        // Create diff between parent and current commit
        let inner = self.repository.inner.diff_tree_to_tree(
            parent.as_ref(),
            Some(&tree),
            Some(&mut git2::DiffOptions::new()),
        )?;

        // Return iterator over deltas
        Ok(Deltas { inner, index: 0 })
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Iterator for Deltas<'_> {
    type Item = Delta;

    /// Returns the next delta.
    fn next(&mut self) -> Option<Self::Item> {
        let delta = self.inner.get_delta(self.index)?;
        self.index += 1;

        // Obtain old and new file paths
        let from = delta.old_file().path()?;
        let path = delta.new_file().path()?;

        // Handle according to type
        match delta.status() {
            // Path was added
            git2::Delta::Added => {
                let path = path.to_path_buf();
                Some(Delta::Create { path })
            }
            // Path was modified
            git2::Delta::Modified => {
                let path = path.to_path_buf();
                Some(Delta::Modify { path })
            }
            // Path was copied or changed type
            git2::Delta::Copied | git2::Delta::Typechange => {
                let path = path.to_path_buf();
                Some(Delta::Modify { path })
            }
            // Path was renamed
            git2::Delta::Renamed => {
                let from = from.to_path_buf();
                let path = path.to_path_buf();
                Some(Delta::Rename { from, path })
            }
            // Path was deleted
            git2::Delta::Deleted => {
                let path = from.to_path_buf();
                Some(Delta::Delete { path })
            }
            // Everything else can be ignored
            _ => self.next(),
        }
    }
}
