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

//! Iterator over resolved paths of a glob.

use glob::glob;
use std::path::PathBuf;

use crate::project::Result;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Iterator over resolved paths of a glob.
#[derive(Debug, Default)]
pub struct Paths {
    /// Stack of patterns.
    patterns: Vec<PathBuf>,
    /// Stack of paths.
    paths: Vec<PathBuf>,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Iterator for Paths {
    type Item = Result<PathBuf>;

    /// Returns the next path.
    fn next(&mut self) -> Option<Self::Item> {
        if self.paths.is_empty() {
            // Take next pattern from the stack and expand it as a glob, or
            // propagate the error in case the pattern is invalid
            let paths = match glob(self.patterns.pop()?.to_str()?) {
                Ok(paths) => paths,
                Err(err) => return Some(Err(err.into())),
            };

            // Collect paths and propagate errors - note that we need to know
            // when an error occurs, so we don't just silence them
            let iter = paths.into_iter().map(|res| res.map_err(Into::into));
            match iter.collect::<Result<Vec<_>>>() {
                Ok(paths) => {
                    // We must make sure that every path is a directory, as we
                    // append the manifest file name later on
                    let iter = paths.into_iter().filter(|path| path.is_dir());
                    self.paths.extend(iter.rev());
                }
                Err(err) => return Some(Err(err)),
            }
        }

        // Return next path
        self.paths.pop().map(Ok)
    }
}

// ----------------------------------------------------------------------------

impl FromIterator<PathBuf> for Paths {
    /// Creates an iterator from an iterator over patterns.
    ///
    /// Note that the given patterns must be valid paths and resolvable from
    /// the current working directory. It's recommended to use absolute paths.
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = PathBuf>,
    {
        Self {
            patterns: iter.into_iter().collect(),
            paths: Vec::new(),
        }
    }
}
