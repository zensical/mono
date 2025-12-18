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

//! Change.

use std::collections::BTreeSet;
use std::fmt::{self, Write};
use std::str::FromStr;

use mono_project::version::Increment;

mod error;
mod kind;

pub use error::{Error, Result};
pub use kind::Kind;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Change.
#[derive(Debug)]
pub struct Change {
    /// Change kind.
    kind: Kind,
    /// Change summary.
    summary: String,
    /// Change references.
    references: Vec<u32>,
    /// Change is breaking.
    is_breaking: bool,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Change {
    /// Returns the corresponding version increment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mono_changeset::{Change, Increment};
    ///
    /// // Create increment from change
    /// let change: Change = "fix: summary".parse()?;
    /// assert_eq!(change.as_increment(), Some(Increment::Patch));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn as_increment(&self) -> Option<Increment> {
        let increment = match self.kind {
            Kind::Feature => Increment::Minor,
            Kind::Fix => Increment::Patch,
            Kind::Performance => Increment::Patch,
            Kind::Refactor => Increment::Patch,
            _ => return None,
        };

        // If a version increment is determined, check for breaking changes,
        // as they must always lead to a major version increment
        if self.is_breaking {
            Some(Increment::Major)
        } else {
            Some(increment)
        }
    }
}

#[allow(clippy::must_use_candidate)]
impl Change {
    /// Returns the change kind.
    #[inline]
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Returns the change summary.
    #[inline]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Returns the change references.
    #[inline]
    pub fn references(&self) -> &[u32] {
        &self.references
    }

    /// Returns whether the change is breaking.
    #[inline]
    pub fn is_breaking(&self) -> bool {
        self.is_breaking
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl FromStr for Change {
    type Err = Error;

    /// Attempts to create a change from a string.
    ///
    /// # Errors
    ///
    /// This methods return [`Error::Format`][], if the string does not adhere
    /// to conventional commits format (without scopes), and [`Error::Kind`][],
    /// if the string does not correspond to a valid [`Kind`] variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mono_changeset::Change;
    ///
    /// // Create change from string
    /// let change: Change = "fix: summary".parse()?;
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(value: &str) -> Result<Self> {
        let Some((kind, summary)) = value.split_once(": ") else {
            return Err(Error::Format);
        };

        // Check if we have a breaking change, denoted by an exclamation mark
        // at the end of the string, and extract and parse the change kind
        let (kind, is_breaking) = match kind.split_once('!') {
            Some((kind, _)) => (Kind::from_str(kind)?, true),
            None => (Kind::from_str(kind)?, false),
        };

        // Ensure summary has no leading or trailing whitespace, as we aim to
        // be as strict as possible with the conventional commits format
        if summary != summary.trim() {
            return Err(Error::Whitespace);
        }

        // Ensure summary is lowercase, unless it's an entire uppercase word,
        // e.g., an acronym like README, API, HTTP, or URL
        if let Some(char) = summary.chars().next() {
            if char.is_uppercase() {
                let word = summary.split_whitespace().next().unwrap_or("");
                let is_acronym = word
                    .chars()
                    .all(|char| !char.is_alphabetic() || char.is_uppercase());

                // If not an acronym, return error
                if !is_acronym {
                    return Err(Error::Casing);
                }
            }
        }

        // Ensure summary does not end with sentenceending punctuation, as the
        // changelog should read as a short list of bullet points
        if summary.ends_with(['.', '!', '?', ',', ';', ':']) {
            return Err(Error::Punctuation);
        }

        // Extract references from the summary, and ensure they are sorted,
        // which is why we use a sorted set instead of a vector here
        let mut references = BTreeSet::new();
        Ok(Change {
            kind,
            summary: extract(summary, &mut references)?,
            references: Vec::from_iter(references),
            is_breaking,
        })
    }
}

// ----------------------------------------------------------------------------

impl fmt::Display for Change {
    /// Formats the change for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)?;
        if self.is_breaking {
            f.write_char('!')?;
        }

        // Write summary
        f.write_str(": ")?;
        self.summary.fmt(f)?;

        // Write references
        if !self.references.is_empty() {
            f.write_str(" (")?;
            for (i, reference) in self.references.iter().enumerate() {
                f.write_char('#')?;
                reference.fmt(f)?;

                // Write comma if not last
                if i < self.references.len() - 1 {
                    f.write_str(", ")?;
                }
            }
            f.write_char(')')?;
        }

        // No errors occurred
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Extracts references in the format `(#123)` from the given string, or fails
/// if a reference was found but it's not wrapped in parenthesis.
fn extract(value: &str, references: &mut BTreeSet<u32>) -> Result<String> {
    let mut summary = Vec::new();

    // Iterate over the given string, searching for references
    let mut start = 0;
    for end in 0..value.len() {
        if end < start {
            continue;
        }

        // Check, if the current character is a '#', which might indicate a
        // reference if and only if followed by numeric characters
        if &value[end..=end] != "#" {
            continue;
        }

        // Now, try to read as many numeric characters as possible after the
        // '#', and consider it a reference if we found any
        let rest = &value[end + 1..];
        let Some(after) = rest.find(|char: char| !char.is_numeric()) else {
            continue;
        };

        // In case we found a reference, parse it, and add it to the list if
        // it is wrapped in parenthesis. Otherwise, fail with an error.
        if after > 0 {
            let Ok(reference) = rest[0..after].parse::<u32>() else {
                continue;
            };

            // Check format if we're not at the beginning of the string
            if end > 0 {
                let opening = value.chars().nth(end - 1);
                let closing = value.chars().nth(end + after + 1);

                // Next, check if the characters before the '#' and after the
                // reference are both parenthesis, as this is required
                if opening == Some('(') && closing == Some(')') {
                    references.insert(reference);
                } else {
                    return Err(Error::Reference);
                }

                // Extract summary part before the reference, and move the
                // start position exactly after the reference
                summary.push(value[start..end - 1].trim());
                start = end + after + 2;
            }
        }
    }

    // Extract remaining part of the summary, and join parts with whitespace
    // to return them as a cleaned up version of the original summary
    summary.push(value[start..].trim());
    Ok(summary.join(" "))
}

// ----------------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    #[allow(clippy::bool_assert_comparison)]
    mod from_str {
        use std::str::FromStr;

        use crate::changeset::change::{Change, Error, Kind, Result};

        #[test]
        fn handles_non_breaking() -> Result {
            let change = Change::from_str("fix: summary")?;
            assert_eq!(change.kind, Kind::Fix);
            assert_eq!(change.is_breaking, false);
            assert_eq!(change.summary, "summary");
            Ok(())
        }

        #[test]
        fn handles_breaking() -> Result {
            let change = Change::from_str("fix!: summary")?;
            assert_eq!(change.kind, Kind::Fix);
            assert_eq!(change.is_breaking, true);
            assert_eq!(change.summary, "summary");
            Ok(())
        }

        #[test]
        fn errors_on_invalid_format() {
            for format in [
                "fix:summary",
                "fix:  summary",
                "fix :summary",
                "fix summary",
            ] {
                let res = Change::from_str(format);
                assert!(matches!(res, Err(Error::Format)));
            }
        }

        #[test]
        fn errors_on_invalid_kind() {
            for format in [
                " fix: summary", // fmt
                "fix : summary",
                "fxi: summary",
            ] {
                let res = Change::from_str(format);
                assert!(matches!(res, Err(Error::Kind)));
            }
        }
    }
}
