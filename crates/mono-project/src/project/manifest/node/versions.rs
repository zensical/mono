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

//! Node manifest update.

use serde_json::{Map, Value};

use crate::project::workspace::Versions;
use crate::project::Result;

use super::Node;

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Versions<'_, Node> {
    /// Updates package versions in the given manifest content.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Json`][] if parsing or printing fails.
    ///
    /// [`Error::Json`]: crate::project::Error::Json
    pub fn update<S>(&self, content: S) -> Result<String>
    where
        S: AsRef<str>,
    {
        let content = content.as_ref();
        let mut doc = content.parse::<Value>()?;

        // Apply updates to the document
        if let Some(map) = doc.as_object_mut() {
            self.update_version(map);
            self.update_dependencies(map);
        }

        // Return updated document - ensure trailing line feed
        let content = serde_json::to_string_pretty(&doc)?;
        Ok(format!("{content}\n"))
    }

    /// Updates `version` with a new version.
    fn update_version(&self, doc: &mut Map<String, Value>) {
        if let Some(name) = doc.get("name").and_then(|item| item.as_str()) {
            if let Some(version) = self.get(name) {
                doc.insert(
                    String::from("version"),
                    Value::String(version.to_string()),
                );
            }
        }
    }

    /// Updates `dependencies` and `devDependencies` with new versions
    fn update_dependencies(&self, doc: &mut Map<String, Value>) {
        for section in ["dependencies", "devDependencies"] {
            if let Some(map) =
                doc.get_mut(section).and_then(|value| value.as_object_mut())
            {
                for (name, value) in map.iter_mut() {
                    if let Some(version) = self.get(name.as_str()) {
                        *value = Value::String(format!("^{version}"));
                    }
                }
            }
        }
    }
}
