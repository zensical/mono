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

//! Cargo manifest update.

use semver::Version;
use toml_edit::{value, DocumentMut, Item};

use crate::project::workspace::Versions;
use crate::project::Result;

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Updates package versions in the given manifest content.
///
/// # Errors
///
/// Returns [`Error::TomlEdit`][] if parsing or printing fails.
///
/// [`Error::TomlEdit`]: crate::project::Error::TomlEdit
pub fn update<S>(content: S, versions: &Versions<'_>) -> Result<String>
where
    S: AsRef<str>,
{
    let content = content.as_ref();
    let mut doc = content.parse::<DocumentMut>()?;

    // Apply updates to the document
    update_package_version(&mut doc, versions);
    update_workspace_dependencies(&mut doc, versions);
    update_dependencies(&mut doc, versions);

    // Return updated document
    Ok(doc.to_string())
}

// ----------------------------------------------------------------------------

/// Updates `[package].version` with a new version.
fn update_package_version(doc: &mut DocumentMut, versions: &Versions<'_>) {
    if let Some(package) = doc
        .get_mut("package")
        .and_then(|item| item.as_table_like_mut())
    {
        if let Some(name) = package.get("name").and_then(|item| item.as_str()) {
            if let Some(version) = versions.get(name) {
                package.insert("version", value(version.to_string()));
            }
        }
    }
}

/// Updates `[workspace.dependencies]` with new versions.
fn update_workspace_dependencies(
    doc: &mut DocumentMut, versions: &Versions<'_>,
) {
    if let Some(table) = doc
        .get_mut("workspace")
        .and_then(|item| item.get_mut("dependencies"))
        .and_then(|item| item.as_table_like_mut())
    {
        for (name, item) in table.iter_mut() {
            if let Some(version) = versions.get(name.get()) {
                update_dependency(item, version);
            }
        }
    }
}

/// Updates `[dependencies]` and `[dev-dependencies]` with new versions.
fn update_dependencies(doc: &mut DocumentMut, versions: &Versions<'_>) {
    for section in ["dependencies", "dev-dependencies"] {
        if let Some(table) = doc
            .get_mut(section)
            .and_then(|item| item.as_table_like_mut())
        {
            for (name, item) in table.iter_mut() {
                if let Some(version) = versions.get(name.get()) {
                    update_dependency(item, version);
                }
            }
        }
    }
}

/// Updates a dependency with a new version.
fn update_dependency(item: &mut Item, version: &Version) {
    if let Some(table) = item.as_table_like() {
        if let Some(workspace) = table.get("workspace") {
            // Skip if dependency inherits from workspace
            if workspace.as_bool() == Some(true) {
                return;
            }
        }
    }

    // Update simple version string: `foo = "1.0.0"`
    if item.is_str() {
        *item = value(version.to_string());

    // Update inline table: `foo = { version = "1.0.0" }`
    } else if let Some(table) = item.as_table_like_mut() {
        table.insert("version", value(version.to_string()));
    }
}
