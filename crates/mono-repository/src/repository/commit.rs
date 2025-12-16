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

//! Commit.

use std::fmt;

use super::error::Result;
use super::id::Id;
use super::Repository;

mod delta;
mod iter;

pub use delta::{Delta, Deltas};
pub use iter::Commits;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Commit.
pub struct Commit<'a> {
    /// Repository.
    repository: &'a Repository,
    /// Git commit.
    inner: git2::Commit<'a>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Repository {
    /// Attempts to find a commit by its identifier.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`][] if the operation fails.
    ///
    /// [`Error::Git`]: crate::repository::Error::Git
    pub fn get<I>(&self, id: I) -> Result<Commit<'_>>
    where
        I: Into<Id>,
    {
        Ok(Commit {
            repository: self,
            inner: self.inner.find_commit(*id.into())?,
        })
    }

    /// Attempts to find a commit by revision specification.
    ///
    /// This method accepts any revision specification as understood by Git,
    /// such as commit hashes, branch names, tags, and more.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`][] if the operation fails.
    ///
    /// [`Error::Git`]: crate::repository::Error::Git
    pub fn find<S>(&self, spec: S) -> Result<Commit<'_>>
    where
        S: AsRef<str>,
    {
        let object = self.inner.revparse_single(spec.as_ref())?;
        Ok(Commit {
            repository: self,
            inner: object.peel_to_commit()?,
        })
    }
}

// ----------------------------------------------------------------------------

#[allow(clippy::must_use_candidate)]
impl Commit<'_> {
    /// Returns the commit identifier.
    #[inline]
    pub fn id(&self) -> Id {
        self.inner.id().into()
    }

    /// Returns the commit summary.
    #[allow(clippy::missing_panics_doc)]
    #[inline]
    pub fn summary(&self) -> &str {
        self.inner.summary().expect("invariant")
    }

    /// Returns the commit body.
    #[inline]
    pub fn body(&self) -> Option<&str> {
        self.inner.body().filter(|body| !body.is_empty())
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl PartialEq for Commit<'_> {
    /// Compares two commits for equality.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.id() == other.inner.id()
    }
}

impl Eq for Commit<'_> {}

// ----------------------------------------------------------------------------

impl fmt::Display for Commit<'_> {
    /// Formats the commit for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.id().fmt(f)
    }
}

impl fmt::Debug for Commit<'_> {
    /// Formats the commit for debugging.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Commit")
            .field("id", &self.id())
            .field("summary", &self.summary())
            .field("body", &self.body())
            .finish()
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Trims the trailers from the given commit message.
///
/// This function implements the most concise way to remove trailers from the
/// given message, e.g., to use the body in a changelog summary.
///
/// # Errors
///
/// This method returns [`Error::Git`][] if the operation fails.
///
/// [`Error::Git`]: crate::repository::Error::Git
pub fn trim_trailers(message: &str) -> Result<&str> {
    // We must add two line feeds to the message, or the trailers would not be
    // discoverable, since git assumes that we pass the entire commit message
    let prepared = format!("\n\n{message}");
    let trailers = git2::message_trailers_strs(prepared.as_str())?;
    if let Some((key, _)) = trailers.iter().next() {
        Ok(message.split_once(key).map_or(message, |(body, _)| body))
    } else {
        Ok(message)
    }
}
