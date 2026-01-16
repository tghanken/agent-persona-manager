---
name: conventional-commits
description: Generates conventional commit messages for PRs and git commits.
---

# Conventional Commits

This skill assists in creating commit messages that follow the Conventional Commits specification.

## Structure

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Common Types

| Type | Description |
|------|-------------|
| `feat` | A new feature |
| `fix` | A bug fix |
| `docs` | Documentation only changes |
| `style` | Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc) |
| `refactor` | A code change that neither fixes a bug nor adds a feature |
| `perf` | A code change that improves performance |
| `test` | Adding missing tests or correcting existing tests |
| `build` | Changes that affect the build system or external dependencies |
| `ci` | Changes to our CI configuration files and scripts |
| `chore` | Other changes that don't modify src or test files |

## Rules

1. **Subject Line**:
   - Use imperative mood ("add" not "added").
   - Don't capitalize the first letter.
   - No dot (.) at the end.
   - Keep it short (under 50 chars preferred).

2. **Body** (Optional):
   - Use imperative mood.
   - Explain *what* and *why* vs *how*.

3. **Breaking Changes**:
   - Append `!` after the type/scope (e.g., `feat!: ...`).
   - Or add `BREAKING CHANGE:` footer.

## Usage

When asking to create a commit message, provide the diff or a summary of changes.
The agent will generate a message following this format.
