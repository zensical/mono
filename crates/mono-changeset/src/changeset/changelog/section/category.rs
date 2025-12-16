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

//! Section category.

use std::fmt;

use crate::changeset::change::Kind;
use crate::changeset::Change;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Section category.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    /// Breaking changes.
    Breaking,
    /// Features.
    Feature,
    /// Bug fixes.
    Fix,
    /// Performance improvements.
    Performance,
    /// Refactorings.
    Refactor,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl From<&Change> for Option<Category> {
    /// Converts a change to a section category.
    ///
    /// This method can be used to check if a change belongs to a [`Section`][]
    /// that will be featured in the [`Changelog`][] to be generated.
    ///
    /// [`Changelog`]: crate::changeset::changelog::Changelog
    /// [`Section`]: crate::changeset::changelog::Section
    fn from(change: &Change) -> Self {
        let category = if change.is_breaking() {
            Category::Breaking
        } else {
            match change.kind() {
                Kind::Feature => Category::Feature,
                Kind::Fix => Category::Fix,
                Kind::Performance => Category::Performance,
                Kind::Refactor => Category::Refactor,
                _ => return None,
            }
        };

        // Return category
        Some(category)
    }
}

// ----------------------------------------------------------------------------

impl fmt::Display for Category {
    /// Formats the section category for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Breaking => f.write_str("Breaking changes"),
            Category::Feature => f.write_str("Features"),
            Category::Fix => f.write_str("Bug fixes"),
            Category::Performance => f.write_str("Performance improvements"),
            Category::Refactor => f.write_str("Refactorings"),
        }
    }
}
