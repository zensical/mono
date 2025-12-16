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

//! Changelog.

use std::collections::BTreeMap;
use std::fmt;

use super::revision::Revision;
use super::scopes::Scopes;
use super::Changeset;

mod section;

pub use section::{Category, Section};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Changelog.
///
/// Changelogs are temporary views on a [`Changeset`][], grouping revisions by
/// category, which is deduced from the [`Kind`] of change, and ignoring any
/// changes irrelevant for versioning. Breaking changes are always grouped
/// into their own section, which comes first.
///
/// The changelog is solely intended for printing, which is why it implements
/// [`fmt::Display`]. The output format is Markdown, as supported by GitHub.
///
/// [`Changeset`]: crate::changeset::Changeset
#[derive(Debug)]
pub struct Changelog<'a> {
    /// Scope set.
    scopes: &'a Scopes,
    /// Sections grouped by category.
    sections: BTreeMap<Category, Section<'a>>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Changeset<'_> {
    /// Creates a changelog from the changeset.
    #[must_use]
    pub fn to_changelog(&self) -> Changelog<'_> {
        let mut changelog = Changelog {
            scopes: &self.scopes,
            sections: BTreeMap::default(),
        };

        // Extend changelog with all revisions
        changelog.extend(&self.revisions);
        changelog
    }
}

// ----------------------------------------------------------------------------

impl<'a> Changelog<'a> {
    /// Adds a revision to the changelog.
    ///
    /// Note that only relevant changes are included in the changelog, which
    /// includes features, fixes, performance improvements and refactorings. In
    /// case the changeset does not include such changes, the changelog will be
    /// empty, which is expected, since no release is necessary.
    pub fn add(&mut self, revision: &'a Revision<'a>) {
        let change = revision.change();

        // Determine section category, create section and add revision - note
        // that we need to pass the scopes for rendering, as only indices are
        // stored, and not all types of changes are featured in the changelog,
        // so we skip those that are not
        if let Some(category) = change.into() {
            self.sections
                .entry(category)
                .or_insert_with(|| category.into())
                .add(revision, self.scopes);
        }
    }
}

#[allow(clippy::must_use_candidate)]
impl Changelog<'_> {
    /// Returns the number of changes.
    #[inline]
    pub fn len(&self) -> usize {
        self.sections.values().map(Section::len).sum()
    }

    /// Returns whether there are any changes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.sections.values().all(Section::is_empty)
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<'a> Extend<&'a Revision<'a>> for Changelog<'a> {
    /// Extends the changelog with the given revisions.
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a Revision<'a>>,
    {
        for revision in iter {
            self.add(revision);
        }
    }
}

// ----------------------------------------------------------------------------

impl fmt::Display for Changelog<'_> {
    /// Formats the changelog for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.sections.is_empty() {
            f.write_str("## Changelog")?;
        }

        // Write all sections
        for section in self.sections.values() {
            f.write_str("\n\n")?;
            section.fmt(f)?;
        }

        // No errors occurred
        Ok(())
    }
}
