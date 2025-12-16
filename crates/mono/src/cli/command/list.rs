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

//! List the names of all packages in topological order.

use clap::Args;

use mono_project::Manifest;

use crate::cli::{Command, Result};
use crate::Context;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// List the names of all packages in topological order.
#[derive(Args, Debug)]
pub struct Arguments {}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<T> Command<T> for Arguments
where
    T: Manifest,
{
    /// Executes the command.
    fn execute(&self, context: Context<T>) -> Result {
        let dependents = context.workspace.dependents()?;
        for node in &dependents {
            let name = dependents[node].name().expect("invariant");
            println!("{name}");
        }

        // No errors occurred
        Ok(())
    }
}
