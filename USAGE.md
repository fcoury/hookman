# Hookman Usage Guide

## Table of Contents

- [Quick Start](#quick-start)
- [Common Workflows](#common-workflows)
- [Hook Types](#hook-types)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Quick Start

1. **Initialize hookman in your project:**
   ```bash
   cd your-project
   hookman init
   ```

2. **Add a pre-commit hook:**
   ```bash
   hookman add pre-commit "cargo fmt -- --check" --id format --description "Check code formatting"
   ```

3. **Apply hooks to Git:**
   ```bash
   hookman apply
   ```

4. **Check status:**
   ```bash
   hookman status
   ```

## Common Workflows

### Setting up code quality checks

```bash
# Initialize hookman
hookman init

# Add formatting check
hookman add pre-commit "cargo fmt -- --check" \
  --id format \
  --description "Check Rust formatting"

# Add linting
hookman add pre-commit "cargo clippy -- -D warnings" \
  --id lint \
  --description "Run Clippy linter"

# Add tests to pre-push
hookman add pre-push "cargo test" \
  --id test \
  --description "Run all tests"

# Apply the hooks
hookman apply
```

### Enforcing commit message format

```bash
# Add commit message validation
hookman add commit-msg 'grep -qE "^(feat|fix|docs|style|refactor|test|chore): " "$1"' \
  --id conventional \
  --description "Enforce conventional commit format"

hookman apply
```

### Sharing hooks with your team

1. Commit the `.hookman` directory:
   ```bash
   git add .hookman
   git commit -m "Add project Git hooks"
   ```

2. Team members can apply hooks after cloning:
   ```bash
   git clone <repository>
   cd <repository>
   hookman apply
   ```

## Hook Types

Hookman supports all standard Git hooks:

### Commit Workflow Hooks
- **pre-commit**: Run before commit is created
- **prepare-commit-msg**: Modify default commit message
- **commit-msg**: Validate commit message
- **post-commit**: Run after commit is created

### Push/Pull Hooks
- **pre-push**: Run before push to remote
- **pre-receive**: Run on remote before receiving push
- **update**: Run on remote for each updated ref
- **post-receive**: Run on remote after receiving push

### Other Hooks
- **pre-rebase**: Run before rebase
- **post-checkout**: Run after checkout
- **post-merge**: Run after merge
- **pre-applypatch**: Run before patch is applied
- **post-applypatch**: Run after patch is applied
- **pre-merge**: Run before merge commit
- **post-rewrite**: Run after commits are rewritten

## Best Practices

### 1. Use descriptive IDs and descriptions
```bash
# Good
hookman add pre-commit "npm run lint" --id eslint --description "Check JavaScript code style"

# Less descriptive
hookman add pre-commit "npm run lint" --id lint
```

### 2. Keep hooks fast
Pre-commit hooks should complete quickly to avoid disrupting workflow. Move slower checks to pre-push or CI.

### 3. Make hooks skippable
For emergency commits, Git allows skipping hooks:
```bash
git commit --no-verify
```

### 4. Test hooks before applying
Use dry-run to preview changes:
```bash
hookman apply --dry-run
```

### 5. Document hook requirements
Add a section to your README explaining required tools:
```markdown
## Development Setup
This project uses Git hooks. After cloning:
1. Install Rust toolchain
2. Run `hookman apply` to install Git hooks
```

## Troubleshooting

### Hook not executing

1. Check if hook is applied:
   ```bash
   hookman status
   ```

2. Verify hook file exists and is executable:
   ```bash
   ls -la .git/hooks/pre-commit
   ```

3. Re-apply hooks:
   ```bash
   hookman apply
   ```

### Command not found errors

Ensure required tools are installed and in PATH. Hooks run in a minimal environment, so you may need to specify full paths:

```bash
# Instead of
hookman add pre-commit "cargo fmt" --id format

# Use
hookman add pre-commit "/usr/local/bin/cargo fmt" --id format
```

### Removing hooks

To temporarily disable hookman hooks:
```bash
rm .git/hooks/*
```

To remove a specific command:
```bash
hookman remove pre-commit format
hookman apply
```

### Debugging hooks

Add echo statements to see what's happening:
```bash
hookman add pre-commit "echo 'Running formatter...'" --id debug1
hookman add pre-commit "cargo fmt -- --check" --id format
hookman add pre-commit "echo 'Format check complete'" --id debug2
```

## Examples

### JavaScript/Node.js project
```bash
hookman init
hookman add pre-commit "npm run lint" --id lint --description "ESLint check"
hookman add pre-commit "npm run prettier:check" --id format --description "Prettier check"
hookman add pre-push "npm test" --id test --description "Run Jest tests"
hookman apply
```

### Python project
```bash
hookman init
hookman add pre-commit "black --check ." --id format --description "Check Black formatting"
hookman add pre-commit "flake8 ." --id lint --description "Flake8 linting"
hookman add pre-commit "mypy ." --id typecheck --description "Type checking"
hookman apply
```

### Go project
```bash
hookman init
hookman add pre-commit "go fmt ./..." --id format --description "Go formatting"
hookman add pre-commit "golint ./..." --id lint --description "Go linting"
hookman add pre-push "go test ./..." --id test --description "Run tests"
hookman apply
```