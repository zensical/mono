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

//! Section item.

use std::fmt::{self, Write};

use crate::changeset::revision::Revision;
use crate::changeset::scopes::Scopes;

use super::Section;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Section item.
#[derive(Debug)]
pub struct Item<'a> {
    /// Revision.
    revision: &'a Revision<'a>,
    /// Affected scopes.
    scopes: Vec<&'a str>,
    /// Relevant issues.
    issues: Vec<u32>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<'a> Section<'a> {
    /// Adds a revision to the section item.
    pub fn add(&mut self, revision: &'a Revision, scopes: &'a Scopes) {
        // Obtain names of affected scopes
        let mut affected = Vec::new();
        for &index in revision.scopes() {
            let (_, name) = &scopes[index];
            affected.push(name.as_str());
        }

        // Create item and add to section
        self.items.push(Item {
            revision,
            scopes: affected,
            issues: revision.issues().to_vec(),
        });
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl fmt::Display for Item<'_> {
    /// Formats the section item for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = self.revision.commit().id();
        id.short().fmt(f)?;

        // Write affected scopes
        if !self.scopes.is_empty() {
            f.write_char(' ')?;
            for (i, scope) in self.scopes.iter().enumerate() {
                f.write_str("__")?;
                f.write_str(scope)?;
                f.write_str("__")?;

                // Write comma if not last
                if i < self.scopes.len() - 1 {
                    f.write_str(", ")?;
                }
            }
        }

        // Retrieve change and write summary
        let change = self.revision.change();
        f.write_str(" – ")?;
        f.write_str(change.summary())?;

        // Write relevant issues
        if !self.issues.is_empty() {
            f.write_str(" (")?;
            for (i, issue) in self.issues.iter().enumerate() {
                f.write_char('#')?;
                issue.fmt(f)?;

                // Write comma if not last
                if i < self.issues.len() - 1 {
                    f.write_str(", ")?;
                }
            }
            f.write_char(')')?;
        }

        // No errors occurred
        Ok(())
    }
}
