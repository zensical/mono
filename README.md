# Mono

`mono` is a release tool for monorepos. It validates commits, derives scopes
from changed files, determines versions, and generates changelogs from git
history. It is the tool we use to release all Zensical projects.

## Installation

``` bash
cargo install mono
```

Or use the GitHub Action:

``` yaml
- uses: zensical/mono@v0
```

## Modes

### Automatic workspace discovery

This is the default mode. If the repository root is a Cargo workspace or a
Node workspace, `mono` discovers packages automatically and uses package names
as scopes.

Cargo:

``` toml
[workspace]
members = ["crates/*"]
```

Node:

``` json
{
  "workspaces": ["packages/*"]
}
```

To lists and release packages, simply run:

``` bash
mono list
mono version create
mono version changelog
```

One of `mono`'s core features is that it deduces the affected scope or scopes
from the changed files in each commit. You do not need to maintain scope data
by hand for normal workspaces.

### Explicit multi-root configuration

Use `.mono.toml` when a repository does not have a single Cargo or Node
workspace root, but still contains multiple releasable leaf projects.

``` toml
[projects]
integration-code = "integrations/code"
integration-neovim = "integrations/neovim"
integration-zed = "integrations/zed"
```

`projects` maps release scopes to explicit project roots.

``` bash
mono list
mono version create
mono version changelog
```

### Additional scopes

Additional scopes can be configured independently of the packaging mode. They
work with both automatic workspace discovery and explicit multi-root
configuration.

``` toml
[scopes]
grammar-textmate = "grammars/textmate/**"
```

These scopes are used for change detection and changelog grouping only. They
do not contribute to version computation and they do not define releasable
packages.

## Non-goals

`.mono.toml` is not intended to become a generic package-management layer.
`mono` already supports additional scopes there, but normal Cargo and Node
workspace discovery remains the preferred model when a regular workspace
exists. One reason for this is that new ecosystems should be as simple as
possible to add: `mono` only needs to learn how to discover a native manifest,
read package names, versions, members, and inner-workspace dependencies, and
update and synchronize that manifest during a release.

More ecosystems may be added over time, but that does not imply a plan to use
`.mono.toml` as a universal format for package and version management across
arbitrary ecosystems.

## Commands

```bash
# List all releasable packages in dependency order
mono list

# Create a release commit and update versions
mono version create

# Print the changelog in Markdown
mono version changelog

# List changed packages since the last release
mono version changed

# Validate a commit by identifier
mono validate commit --id 7b5e433
```
