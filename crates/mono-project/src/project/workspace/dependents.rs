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

//! Workspace dependents.

use std::ops::Index;
use zrx::graph::iter::{Sinks, Sources};
use zrx::graph::traversal::IntoIter;
use zrx::graph::Graph;

use crate::project::manifest::Manifest;
use crate::project::Project;

use super::{Result, Workspace};

mod suggestion;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Workspace dependents.
#[derive(Debug)]
pub struct Dependents<'a, T>
where
    T: Manifest,
{
    /// Workspace graph.
    graph: Graph<&'a Project<T>>,
    /// Workspace scopes.
    scopes: Vec<&'a str>,
}

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

impl<T> Workspace<T>
where
    T: Manifest,
{
    /// Returns the dependents of a workspace.
    ///
    /// This method creates a graph that links all projects in a workspace with
    /// their inner-workspace dependencies, allowing to perform a topological
    /// traversal, as handling projects in dependency order is essential for
    /// correct versioning and release management.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Graph`][] if the graph could not be constructed, which
    /// should theoretically and practically never happen.
    ///
    /// [`Error::Graph`]: crate::project::workspace::Error::Graph
    pub fn dependents(&self) -> Result<Dependents<'_, T>> {
        let mut builder = Graph::builder();

        // Add the projects to the graph in the same order as scope-indexed
        // increments, keeping scope labels aligned with graph node indices
        let mut scopes = Vec::new();
        for (scope, path) in &self.scopes {
            if let Some(project) = self.projects.get(path) {
                builder.add_node(project);
                scopes.push(scope.as_str());
            }
        }

        // Analyze the dependencies between projects by resolving manifest
        // dependency names to projects that are part of the workspace
        let mut edges = Vec::new();
        for (n, project) in builder.nodes().iter().enumerate() {
            for name in project.manifest.dependencies() {
                let Some(dependency) = self.get(name) else {
                    continue;
                };

                // Link the dependency to the current project by node index.
                let mut iter = builder.nodes().iter();
                if let Some(m) = iter.position(|&next| next == dependency) {
                    edges.push((n, m));
                }
            }
        }

        // Create links between projects and their dependencies by adding all
        // collected edges to the graph. Note that links are inverted, so that
        // they point from dependencies to dependents, allowing for topological
        // traversal that visits dependencies first.
        for (n, m) in edges {
            builder.add_edge(m, n)?;
        }

        // Create and return dependents
        Ok(Dependents { graph: builder.build(), scopes })
    }
}

// ----------------------------------------------------------------------------

#[allow(clippy::must_use_candidate)]
impl<T> Dependents<'_, T>
where
    T: Manifest,
{
    /// Creates an iterator over the projects.
    #[inline]
    pub fn iter(&self) -> IntoIter {
        self.into_iter()
    }

    /// Creates an iterator over the projects with no dependencies.
    #[inline]
    pub fn sources(&self) -> Sources<'_> {
        self.graph.sources()
    }

    /// Creates an iterator over the projects with no dependents.
    #[inline]
    pub fn sinks(&self) -> Sinks<'_> {
        self.graph.sinks()
    }

    /// Returns the scope of the project at the given index.
    #[inline]
    pub fn scope(&self, index: usize) -> &str {
        self.scopes[index]
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<'a, T> Index<usize> for Dependents<'a, T>
where
    T: Manifest,
{
    type Output = &'a Project<T>;

    /// Returns the project at the given index.
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.graph[index]
    }
}

// ----------------------------------------------------------------------------

impl<'a, T> IntoIterator for &'a Dependents<'a, T>
where
    T: Manifest,
{
    type Item = usize;
    type IntoIter = IntoIter;

    /// Creates an iterator over the projects.
    fn into_iter(self) -> Self::IntoIter {
        self.graph
            .traverse(self.graph.sources().collect::<Vec<_>>())
            .into_iter()
    }
}
