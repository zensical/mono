# Mono

**Mono repository automation toolkit**

`mono` is a release management tool built specifically for monorepos. It validates commits, determines versions, generates changelogs, and orchestrates publishing – all with zero configuration needed for standard workflows. Rust and Node workspaces are supported out of the box, with more languages coming soon.

## Installation

```bash
cargo install mono
```

Or use our GitHub Action:

```yaml
- uses: zensical/mono@v0
```

## Usage

### Version management

```bash
# Create a new version and update all packages
mono version create

# Generate the changelog of a version in Markdown format
mono version changelog

# List the names of changed packages in topological order
mono version changed

# List versions in reverse chronological order
mono version list
```

### Package discovery

```bash
# List the names of all packages in topological order
mono list
```

### Commit validation

```bash
# Validate a commit message summary
mono validate commit "feature: add authentication"

# Validate a commit message in a file
mono validate commit --file .git/COMMIT_EDITMSG

# Validate a commit by identifier
mono validate commit --id 7b5e433
```

## Features

### Monorepo-first design

- **Automatic package discovery** from `Cargo.toml` workspaces or `package.json` workspaces
- **Automatic scope detection** from directory structure based on packages
- **Topological sorting** ensures dependencies are published before dependents

### Conventional commits validation

- Validates commit messages against Conventional Commits format
- Suggests valid scopes based on actual packages in your monorepo
- Works as a git hook or in CI
- Clear, actionable error messages

### Multi-language support

- **Rust** (Cargo workspaces)
- **Node.js** (npm/pnpm/yarn workspaces)
- **More coming:** Python, ...

### Interactive version bumping

- Visual prompt showing suggested version increments
- Computes suggestions from Conventional Commits types
- Understands `0.0.z` (patch-only) and `0.y.z` (breaking changes = minor) ranges
- Batch version bumping for related packages

### Intelligent changelog generation

- Generates changelogs from Conventional Commits
- Groups changes by package and type (Features, Fixes, Breaking Changes)
- Supports **changelog summaries** attached to commits for curated release notes
- Links to issues and pull requests automatically

### Change detection

- Detects which packages changed since last release
- Returns packages in topological order for publishing
- Integrates with `cargo publish`, `npm publish`, or custom scripts

## Comparison

| Tool | Focus | Why mono  |
|------|-------|------------|
| **Lerna** | JavaScript monorepos, publishing | Language-agnostic, opinionated defaults, no Node.js required |
| **changesets** | Developer-written changelogs | Fully automated from git history, no manual changeset files |
| **cocogitto** | Git Conventional Commits | Monorepo-first with automatic scopes and multi-package releases |
| **git-cliff** | Changelog generation | Interactive version selection, package-aware changelogs, topological publishing |
| **cargo-release** | Single Rust crate releases | Multi-package workspaces with dependency ordering |
| **semantic-release** | Automated releases | Simpler, faster, designed for monorepo batch operations |
