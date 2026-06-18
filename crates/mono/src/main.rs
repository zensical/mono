// Copyright (c) 2025-2026 Zensical and contributors

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

//! Command line interface.

use clap::Parser;
use std::fs;

use mono_changeset::Scopes;
use mono_project::{Cargo, Manifest, Node, Project, Workspace};
use mono_repository::Repository;

mod cli;

use cli::{Cli, Config, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Command line context.
#[derive(Debug)]
pub struct Context<T>
where
    T: Manifest,
{
    /// Repository.
    repository: Repository,
    /// Workspace.
    workspace: Workspace<T>,
    /// Scopes.
    scopes: Scopes,
}

// ----------------------------------------------------------------------------
// Program
// ----------------------------------------------------------------------------

/// Entry point.
fn main() -> Result {
    let cli = Cli::parse();
    let repository = Repository::open(&cli.directory)?;
    let path = repository.path();

    // Try to load configuration, if any
    let config_path = path.join(".mono.toml");
    let config = if config_path.exists() {
        let contents = fs::read_to_string(&config_path)?;
        toml::from_str(&contents)?
    } else {
        Config::default()
    };

    // Initialize workspace from projects configuration
    if !config.projects.is_empty() {
        let mut workspace = Workspace::builder(path.canonicalize()?);
        for (scope, root) in &config.projects {
            workspace.add(scope, Project::resolve(path.join(root))?);
        }

        // Build workspace and execute command
        let workspace = workspace.build()?;
        cli.execute(repository, workspace, &config);
        return Ok(());
    }

    // Initialize workspace from manifest files
    if let Ok(workspace) = Workspace::<Cargo>::resolve(path) {
        cli.execute(repository, workspace, &config);
        return Ok(());
    } else if let Ok(workspace) = Workspace::<Node>::resolve(path) {
        cli.execute(repository, workspace, &config);
        return Ok(());
    }

    // No errors occurred
    Ok(())
}
