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

//! Version increment suggestion.

use std::collections::BTreeSet;

use crate::project::manifest::Manifest;
use crate::project::version::Increment;
use crate::project::{Project, Result};

use super::Dependents;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Version increment suggestion.
pub struct Suggestion<'a, T>
where
    T: Manifest,
{
    /// Project.
    project: &'a Project<T>,
    /// Version increment suggestions.
    increments: &'a [Option<Increment>],
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<T> Dependents<'_, T>
where
    T: Manifest,
{
    /// Invokes the given function with version increment suggestions.
    ///
    /// This method propagates version increments through the workspace graph,
    /// starting from the packages with explicit version increments. The given
    /// function is invoked for each package in topological order, passing it a
    /// suggestion with a project and a set of version increment suggestions.
    ///
    /// # Errors
    ///
    /// This method passes through errors returned by the given function.
    pub fn bump<F>(&self, increments: &mut [Option<Increment>], f: F) -> Result
    where
        F: Fn(Suggestion<'_, T>) -> Result<Option<Increment>>,
    {
        // Determine the node indices of all packages with increments, as those
        // are the nodes from which we start the topological traversal of the
        // workspace graph. If there're dependencies between those packages,
        // the traversal ensures that their order is respected as well.
        let iter = increments.iter().enumerate();
        let sources =
            iter.filter_map(|(index, increment)| increment.map(|_| index));

        // Traverse the graph in topological order, so version increments as
        // chosen by the caller are correctly propagated to dependents
        let incoming = self.graph.topology().incoming();
        for node in self.graph.traverse(sources) {
            // Obtain the unique version increments of all dependencies, and
            // collect them into a set for selection through the caller
            let mut options = BTreeSet::from_iter([increments[node]]);
            for &dependency in &incoming[node] {
                if increments[dependency] > increments[node] {
                    options.insert(increments[dependency]);
                }
            }

            // Collect the suggested version increments, and invoke the given
            // function, remembering the returned version increment
            increments[node] = f(Suggestion {
                project: self.graph[node],
                increments: &options.into_iter().collect::<Vec<_>>(),
            })?;
        }

        // No errors occurred
        Ok(())
    }
}

// ----------------------------------------------------------------------------

#[allow(clippy::must_use_candidate)]
impl<T> Suggestion<'_, T>
where
    T: Manifest,
{
    /// Returns a reference to the project.
    #[inline]
    pub fn project(&self) -> &Project<T> {
        self.project
    }

    /// Returns a reference to the version increment suggestions.
    #[inline]
    pub fn increments(&self) -> &[Option<Increment>] {
        self.increments
    }
}
