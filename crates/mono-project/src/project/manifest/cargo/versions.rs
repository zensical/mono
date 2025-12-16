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

use super::Cargo;

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Versions<'_, Cargo> {
    /// Updates package versions in the given manifest content.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::TomlEdit`][] if parsing or printing fails.
    ///
    /// [`Error::TomlEdit`]: crate::project::Error::TomlEdit
    pub fn update<S>(&self, content: S) -> Result<String>
    where
        S: AsRef<str>,
    {
        let content = content.as_ref();
        let mut doc = content.parse::<DocumentMut>()?;

        // Apply updates to the document
        self.update_package_version(&mut doc);
        self.update_workspace_dependencies(&mut doc);
        self.update_dependencies(&mut doc);

        // Return updated document
        Ok(doc.to_string())
    }

    /// Updates `[package].version` with a new version.
    fn update_package_version(&self, doc: &mut DocumentMut) {
        if let Some(package) = doc
            .get_mut("package")
            .and_then(|item| item.as_table_like_mut())
        {
            if let Some(name) =
                package.get("name").and_then(|item| item.as_str())
            {
                if let Some(version) = self.get(name) {
                    package.insert("version", value(version.to_string()));
                }
            }
        }
    }

    /// Updates `[workspace.dependencies]` with new versions.
    fn update_workspace_dependencies(&self, doc: &mut DocumentMut) {
        if let Some(table) = doc
            .get_mut("workspace")
            .and_then(|item| item.get_mut("dependencies"))
            .and_then(|item| item.as_table_like_mut())
        {
            for (name, item) in table.iter_mut() {
                if let Some(version) = self.get(name.get()) {
                    self.update_dependency(item, version);
                }
            }
        }
    }

    /// Updates `[dependencies]` and `[dev-dependencies]` with new versions.
    fn update_dependencies(&self, doc: &mut DocumentMut) {
        for section in ["dependencies", "dev-dependencies"] {
            if let Some(table) = doc
                .get_mut(section)
                .and_then(|item| item.as_table_like_mut())
            {
                for (name, item) in table.iter_mut() {
                    if let Some(version) = self.get(name.get()) {
                        self.update_dependency(item, version);
                    }
                }
            }
        }
    }

    /// Updates a dependency with a new version.
    #[allow(clippy::unused_self)]
    fn update_dependency(&self, item: &mut Item, version: &Version) {
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
}
