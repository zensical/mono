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

//! Repository.

use std::fmt;
use std::path::Path;
use std::process::Command;

pub mod commit;
pub mod commits;
mod error;
pub mod id;
pub mod versions;

pub use error::{Error, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Repository.
pub struct Repository {
    /// Git repository.
    inner: git2::Repository,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Repository {
    /// Finds and open a repository starting from the given path.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`] if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mono_repository::Repository;
    ///
    /// // Find and open repository from current directory
    /// let repo = Repository::open(".")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(Self {
            inner: git2::Repository::discover(path)?,
        })
    }

    /// Stages all files matching the given path specification.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`] if the operation fails.
    pub fn add<S>(&self, spec: S) -> Result
    where
        S: AsRef<str>,
    {
        let mut index = self.inner.index()?;
        index.add_all([spec.as_ref()], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        // No errors occurred
        Ok(())
    }

    /// Commits the staged changes with the given message.
    ///
    /// Note that this method can't use the [`git2`] crate's `commit` logic, as
    /// this makes it impossible to sign commits using GPG. For this reason, we
    /// need to fallback to the `git` command line interface, committing the
    /// changes the regular way.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`] if the operation fails.
    pub fn commit<M>(&self, message: M) -> Result
    where
        M: AsRef<str>,
    {
        let status = Command::new("git")
            .current_dir(self.path())
            .args([
                "commit",
                "--cleanup=verbatim", // Preserve markdown formatting
                "--signoff",          // Add `Signed-off-by` trailer
                "--no-verify",        // Don't run commit hooks
                "--message",
                message.as_ref(),
            ])
            .status()?;

        // Wrap non-zero exit status as error - switch to `ExitStatusError` when
        // #84908 is stable – https://github.com/rust-lang/rust/issues/84908
        if !status.success() {
            return Err(Error::Status(status));
        }

        // No errors occurred
        Ok(())
    }

    /// Creates a new branch with the given name from the current `HEAD`.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`] if the operation fails.
    #[allow(clippy::missing_panics_doc)]
    pub fn branch<N>(&self, name: N) -> Result
    where
        N: AsRef<str>,
    {
        // Retrieve current `HEAD` commit and create new branch
        let commit = self.inner.head()?.peel_to_commit()?;
        let branch = self.inner.branch(name.as_ref(), &commit, false)?;

        // Retrieve branch name, set `HEAD` and switch to it
        let name = branch.get().name().expect("invariant");
        self.inner.set_head(name)?;
        self.inner.checkout_head(None)?;

        // No errors occurred
        Ok(())
    }

    /// Returns whether there are no uncommitted or untracked changes.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`] if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mono_repository::Repository;
    ///
    /// // Find and open repository from current directory
    /// let repo = Repository::open(".")?;
    /// if !repo.is_clean()? {
    ///     println!("Working directory contains uncommitted changes");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_clean(&self) -> Result<bool> {
        let mut options = git2::StatusOptions::new();
        options
            .include_ignored(false)
            .include_untracked(true)
            .recurse_untracked_dirs(true);

        // Retrieve status of git repository
        let statuses = self.inner.statuses(Some(&mut options))?;
        Ok(statuses.is_empty())
    }

    /// Returns whether the current branch is the default branch.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`] if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use mono_repository::Repository;
    ///
    /// // Find and open repository from current directory
    /// let repo = Repository::open(".")?;
    /// if !repo.on_default_branch()? {
    ///     println!("Not on default branch");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn on_default_branch(&self) -> Result<bool> {
        let opt = self.inner.head()?;
        Ok(opt
            .shorthand()
            .filter(|name| ["master", "main"].contains(name))
            .is_some())
    }
}

#[allow(clippy::must_use_candidate)]
impl Repository {
    /// Returns a reference to the repository path.
    #[allow(clippy::missing_panics_doc)]
    #[inline]
    pub fn path(&self) -> &Path {
        self.inner.path().parent().expect("invariant")
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl fmt::Debug for Repository {
    /// Formats the repository for debugging.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Repository")
            .field("path", &self.path())
            .finish()
    }
}
