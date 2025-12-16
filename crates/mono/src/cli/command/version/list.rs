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

//! List versions in reverse chronological order.

use clap::Args;
use std::fmt::Debug;

use mono_project::Manifest;

use crate::cli::{Command, Result};
use crate::Context;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// List versions in reverse chronological order.
#[derive(Args, Debug)]
pub struct Arguments {
    /// Show only the latest version.
    #[arg(short, long)]
    latest: bool,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<T> Command<T> for Arguments
where
    T: Manifest,
{
    /// Executes the command.
    fn execute(&self, context: Context<T>) -> Result {
        // Resolve and list all versions, and abort after writing the latest
        // version to standard out if only the latest version is requested
        let versions = context.repository.versions()?;
        for (version, _) in &versions {
            println!("v{version}");
            if self.latest {
                break;
            }
        }

        // No errors occurred
        Ok(())
    }
}
