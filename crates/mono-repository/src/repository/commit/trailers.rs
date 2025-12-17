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

//! Commit trailer set.

use std::fmt;
use std::str::FromStr;

use crate::repository::commit::Commit;
use crate::repository::{Error, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Commit trailer set.
pub struct Trailers {
    /// Git trailers.
    inner: git2::MessageTrailersStrs,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Commit<'_> {
    /// Returns the trailer set of the commit.
    ///
    /// We must add two line feeds to the message, or the trailers would not be
    /// discoverable, since git assumes that we pass the entire commit message
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`] if the operation fails.
    #[allow(clippy::missing_panics_doc)]
    #[inline]
    pub fn trailers(&self) -> Result<Trailers> {
        Trailers::from_str(self.inner.body().unwrap_or_default())
    }
}

// ----------------------------------------------------------------------------

impl Trailers {
    /// Returns a reference to the value identified by the key.
    pub fn get<K>(&self, key: K) -> Option<&str>
    where
        K: AsRef<str>,
    {
        let mut iter = self.inner.iter();
        iter.find_map(|(candidate, value)| {
            (candidate == key.as_ref()).then_some(value)
        })
    }

    /// Returns whether the commit trailer set contains the key.
    pub fn contains_key<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        let mut iter = self.inner.iter();
        iter.any(|(candidate, _)| candidate == key.as_ref())
    }

    /// Creates an iterator over the commit trailer set.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> git2::MessageTrailersStrsIterator<'_> {
        self.into_iter()
    }
}

#[allow(clippy::must_use_candidate)]
impl Trailers {
    /// Returns the number of trailers.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns whether there are any trailers.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl FromStr for Trailers {
    type Err = Error;

    /// Attempts to create a commit trailer set from a string.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`] if the operation fails.
    fn from_str(value: &str) -> Result<Self> {
        // We must add two line feeds to the message or the trailers wouldn't be
        // discoverable, as git assumes that we pass the entire commit message
        let prepared = format!("\n\n{value}");
        Ok(Self {
            inner: git2::message_trailers_strs(prepared.as_str())?,
        })
    }
}

// ----------------------------------------------------------------------------

impl<'a> IntoIterator for &'a Trailers {
    type Item = (&'a str, &'a str);
    type IntoIter = git2::MessageTrailersStrsIterator<'a>;

    /// Creates an iterator over the commit trailer set.
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

// ----------------------------------------------------------------------------

impl fmt::Debug for Trailers {
    /// Formats the commit trailer set for debugging.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self).finish()
    }
}
