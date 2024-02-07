# git-flow.rs

Extensible git flow written in rust.

**Extensible:** Customize the workflow that suits your preferences.

**Follow configuration:** Standardize team protocols.

## Installation

```sh
cargo install git-flow-rs
```

Or download released binary.

## Usage

`git flow --help`

> Make sure that you have installed git.

```
Usage: git-flow [OPTIONS] <COMMAND>

Commands:
  start   start a task
  finish  finish a task
  drop    drop a task
  track   track a task
  sync    sync branches
  list    list avaliable branch types
  check   check config
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>
  -h, --help           Print help
  -V, --version        Print version
```

A small example.

```sh
# start a feature
git flow start something feature
# or git flow start feature/something
# then branch feature/something created from dev

# implement the feature
# commit changes

# finish the feature
git flow finish feature/something
# then feature/something merged into dev and this branch deleted
```

## Config

Global config file should be located at `~/.config/git-flow/config.toml`(or `C:\Users\YourUsername\AppData\Roaming\git-flow\config.toml` on windows).

Local config file should be located at `<GitRoot>/.git-flow.toml`.

There is no default configuration. Here is an example.

> Avaliable strategy: `merge`, `rebase`, `cherry-pick`.

> Avaliable hook: `before_start`, `after_start`, `before_finish`, `after_finish`, `before_drop`, `after_drop`.

> Regex is avaliable on `to.n.name`.

```toml
[[branch_types]]
name = "feature"
create = "feature/{NAME}"
from = "dev"
to = [{ name = "dev", strategy = "merge" }]
after_start = { command = "git", args = [
  "push",
  "origin",
  "feature/{NAME}:feature/{NAME}",
] }
after_finish = { command = "git", args = [
  "push",
  "origin",
  "--delete",
  "feature/{NAME}",
] }

[[branch_types]]
name = "hotfix"
create = "hotfix/{NAME}"
from = "main"
to = [
  { name = "main", strategy = "merge" },
  { name = "dev", strategy = "merge" },
  { name = "feature/*", strategy = "merge" },
]

[[branch_types]]
name = "bugfix"
create = "bugfix/{NAME}"
from = "dev"
to = [
  { name = "dev", strategy = "merge" },
  { name = "feature/*", strategy = "merge" },
]

[[branch_types]]
name = "release"
create = "release/{NAME}"
from = "dev"
to = [{ name = "main", strategy = "merge" }]
```
