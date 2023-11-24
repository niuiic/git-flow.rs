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
Extensible git flow written in rust.

Usage: git flow <command>

Avaliable commands:

-h, --help
	Print help
-v, --version
	Print version
start (<branch_type> <branch_name>)/(<full_branch_name>)
	start a task
finish (<branch_type> <branch_name>)/(<full_branch_name>)
	finish a task
drop (<branch_type> <branch_name>)/(<full_branch_name>)
	give up a task
track (<branch_type> <branch_name>)/(<full_branch_name>)
	track a task
sync remote/local [--override]
	sync branches to remote/local

Configured branch types:

feature
	from dev to dev
...
```

A small example.

```sh
# start a feature
git flow start feature something
# or git flow start feature/something
# then branch feature/something created from dev

# implement the feature
# commit changes

# finish the feature
git flow finish feature/something
# then feature/something merged into dev and this branch deleted
```

## Config

Global config file should be located at `~/.config/git-flow/.git-flow.json`(or `C:\Users\YourUsername\AppData\Roaming\git-flow\.git-flow.json` on windows).

Local config file should be located at `<GitRoot>/.git-flow.json`.

There is no default configuration. Here is an example.

> Avaliable strategy: `merge`, `rebase`, `cherry-pick`.

> Avaliable hook: `before_start`, `after_start`, `before_finish`, `after_finish`.

> Regex is avaliable on `to.n.branch`.

```json
[
  {
    "type": "feature",
    "name": "feature/{new_branch}",
    "from": "dev",
    "to": [
      {
        "branch": "dev",
        "strategy": "merge"
      }
    ],
    "hooks": {
      "after_start": {
        "command": "git",
        "args": ["push", "origin", "feature/{new_branch}:feature/{new_branch}"]
      },
      "after_finish": {
        "command": "git",
        "args": ["push", "origin", "--delete", "feature/{new_branch}"]
      }
    }
  },
  {
    "type": "hotfix",
    "name": "hotfix/{new_branch}",
    "from": "main",
    "to": [
      {
        "branch": "main",
        "strategy": "merge"
      },
      {
        "branch": "dev",
        "strategy": "merge"
      },
      {
        "branch": "feature/*",
        "strategy": "merge"
      }
    ]
  },
  {
    "type": "bugfix",
    "name": "bugfix/{new_branch}",
    "from": "dev",
    "to": [
      {
        "branch": "dev",
        "strategy": "merge"
      },
      {
        "branch": "feature/*",
        "strategy": "merge"
      }
    ]
  },
  {
    "type": "release",
    "name": "release/{new_branch}",
    "from": "dev",
    "to": [
      {
        "branch": "main",
        "strategy": "merge"
      }
    ]
  }
]
```
