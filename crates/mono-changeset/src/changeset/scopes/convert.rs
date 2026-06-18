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

//! Scope set conversions.

use mono_project::{Manifest, Workspace};

use super::builder::Builder;
use super::error::Result;
use super::Scopes;

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

/// Attempt conversion into [`Scopes`].
///
/// This trait is primarily provided for a more convenient API when creating
/// changesets. It allows callers to pass either existing [`Scopes`], a scope
/// [`Builder`], or a [`Workspace`] from which the default scopes are derived
/// automatically.
pub trait TryIntoScopes {
    /// Attempts to convert into a scope set.
    ///
    /// # Errors
    ///
    /// In case conversion fails, an error should be returned.
    fn try_into_scopes(self) -> Result<Scopes>;
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl TryIntoScopes for Scopes {
    /// Creates a scope set.
    #[inline]
    fn try_into_scopes(self) -> Result<Scopes> {
        Ok(self)
    }
}

impl TryIntoScopes for Builder {
    /// Creates a scope set from a scope set builder.
    #[inline]
    fn try_into_scopes(self) -> Result<Scopes> {
        self.build()
    }
}

impl<T> TryIntoScopes for &Workspace<T>
where
    T: Manifest,
{
    /// Creates a scope set from a workspace reference.
    fn try_into_scopes(self) -> Result<Scopes> {
        let mut builder = Scopes::builder();
        for (path, name) in self.scopes() {
            builder.add(path.join("**"), name)?;
        }

        // Create scope set from builder
        builder.build()
    }
}
