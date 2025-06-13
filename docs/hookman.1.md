# hookman(1) - Git hooks manager

## SYNOPSIS

**hookman** [*COMMAND*] [*OPTIONS*]

## DESCRIPTION

Hookman is a command-line tool for managing Git hooks across projects with a simple, declarative configuration. It allows teams to define, version, and share Git hooks through a `.hookman` directory that can be committed to version control.

## COMMANDS

**init**
    Initialize a new .hookman directory in the current Git repository.

**add** *HOOK_TYPE* *COMMAND* **--id** *ID* [**--description** *DESC*]
    Add a command to a specific hook type. Each command must have a unique ID within the hook.

**remove** *HOOK_TYPE* *COMMAND_ID*
    Remove a command from a specific hook type by its ID.

**list** [*HOOK_TYPE*]
    List all configured hooks, or commands for a specific hook type.

**apply** [**--dry-run**]
    Apply the hook configuration to the Git repository. Use --dry-run to preview changes.

**status**
    Show the current status of hook configurations.

**help** [*COMMAND*]
    Display help information for hookman or a specific command.

## OPTIONS

**-h**, **--help**
    Print help information

**-V**, **--version**
    Print version information

## HOOK TYPES

Hookman supports all standard Git hook types:

- pre-commit
- pre-push
- commit-msg
- post-commit
- pre-rebase
- post-checkout
- post-merge
- pre-receive
- update
- post-receive
- prepare-commit-msg
- post-update
- pre-applypatch
- post-applypatch
- pre-merge-commit
- post-rewrite

## FILES

**.hookman/**
    Directory containing hook configurations

**.hookman/config.toml**
    Hookman configuration file

**.hookman/hooks/*.toml**
    Individual hook configuration files

**.git/hooks/**
    Git hooks directory where scripts are installed

## EXAMPLES

Initialize hookman in a repository:

    $ hookman init

Add a formatting check to pre-commit:

    $ hookman add pre-commit "cargo fmt -- --check" --id format --description "Check formatting"

Add a test runner to pre-push:

    $ hookman add pre-push "cargo test" --id test --description "Run tests"

List all configured hooks:

    $ hookman list

Apply hooks to the repository:

    $ hookman apply

Check current status:

    $ hookman status

## EXIT STATUS

**0**
    Success

**1**
    General error

**2**
    Not in a Git repository

**3**
    Hookman not initialized

## ENVIRONMENT

Hookman does not use any environment variables directly. Git hooks are executed in the environment provided by Git.

## SEE ALSO

git-hooks(5), git(1)

## BUGS

Report bugs at: https://github.com/fcoury/hookman/issues

## AUTHOR

Written by Felipe Coury and contributors.

## COPYRIGHT

Copyright (C) 2024. Licensed under the MIT License.