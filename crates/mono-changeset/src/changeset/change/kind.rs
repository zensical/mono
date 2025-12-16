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

//! Change kind.

use std::fmt;
use std::str::FromStr;

use super::error::{Error, Result};

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Change kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Feature.
    Feature,
    /// Bugfix.
    Fix,
    /// Performance improvement.
    Performance,
    /// Refactoring.
    Refactor,
    /// Build.
    Build,
    /// Documentation.
    Docs,
    /// Formatting.
    Style,
    /// Test.
    Test,
    /// Chore.
    Chore,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Kind {
    /// Valid change kind values.
    pub const VALUES: [Kind; 9] = [
        Kind::Feature,
        Kind::Fix,
        Kind::Performance,
        Kind::Refactor,
        Kind::Build,
        Kind::Docs,
        Kind::Style,
        Kind::Test,
        Kind::Chore,
    ];
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl FromStr for Kind {
    type Err = Error;

    /// Attempts to create a change kind from a string.
    ///
    /// # Errors
    ///
    /// This methods return [`Error::Kind`], if the string does not correspond
    /// to a valid [`Kind`] variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mono_changeset::change::Kind;
    ///
    /// // Create change kind from string
    /// let kind: Kind = "fix".parse()?;
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(value: &str) -> Result<Self> {
        match value {
            "feature" => Ok(Kind::Feature),
            "fix" => Ok(Kind::Fix),
            "performance" => Ok(Kind::Performance),
            "refactor" => Ok(Kind::Refactor),
            "build" => Ok(Kind::Build),
            "docs" => Ok(Kind::Docs),
            "style" => Ok(Kind::Style),
            "test" => Ok(Kind::Test),
            "chore" => Ok(Kind::Chore),
            _ => Err(Error::Kind),
        }
    }
}

// ----------------------------------------------------------------------------

impl fmt::Display for Kind {
    /// Formats the change kind for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Feature => f.write_str("feature"),
            Kind::Fix => f.write_str("fix"),
            Kind::Performance => f.write_str("performance"),
            Kind::Refactor => f.write_str("refactor"),
            Kind::Build => f.write_str("build"),
            Kind::Docs => f.write_str("docs"),
            Kind::Style => f.write_str("style"),
            Kind::Test => f.write_str("test"),
            Kind::Chore => f.write_str("chore"),
        }
    }
}

// ----------------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    mod from_str {
        use std::str::FromStr;

        use crate::changeset::change::{Error, Kind, Result};

        #[test]
        fn handles_valid_variants() -> Result {
            for (value, kind) in [
                ("feature", Kind::Feature),
                ("fix", Kind::Fix),
                ("performance", Kind::Performance),
                ("refactor", Kind::Refactor),
                ("build", Kind::Build),
                ("docs", Kind::Docs),
                ("style", Kind::Style),
                ("test", Kind::Test),
                ("chore", Kind::Chore),
            ] {
                assert_eq!(Kind::from_str(value)?, kind);
            }
            Ok(())
        }

        #[test]
        fn errors_on_invalid_variant() {
            for value in ["feat", "fi x", "perf", "doc", "testing"] {
                let res = Kind::from_str(value);
                assert!(matches!(res, Err(Error::Kind)));
            }
        }

        #[test]
        fn errors_on_invalid_casing() {
            for value in ["FEATURE", "Fix"] {
                let res = Kind::from_str(value);
                assert!(matches!(res, Err(Error::Kind)));
            }
        }
    }
}
