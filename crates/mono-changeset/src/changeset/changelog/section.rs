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

//! Section.

use std::fmt::{self, Write};

mod category;
mod item;

pub use category::Category;
pub use item::Item;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Section.
#[derive(Debug)]
pub struct Section<'a> {
    /// Section category.
    category: Category,
    /// Section items.
    items: Vec<Item<'a>>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

#[allow(clippy::must_use_candidate)]
impl Section<'_> {
    /// Returns the number of items.
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns whether there are any items.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl From<Category> for Section<'_> {
    /// Creates a section from a category.
    #[inline]
    fn from(category: Category) -> Self {
        Self { category, items: Vec::new() }
    }
}

// ----------------------------------------------------------------------------

impl fmt::Display for Section<'_> {
    /// Formats the section for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("### ")?;
        self.category.fmt(f)?;
        f.write_char('\n')?;

        // Write all items, each on a new line
        for item in &self.items {
            f.write_char('\n')?;
            f.write_str("- ")?;
            item.fmt(f)?;
        }

        // No errors occurred
        Ok(())
    }
}
