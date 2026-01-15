# Skill Input Format Specification

This document specifies the expected format for agent skills provided as input to the Agent Knowledge Manager.

## Directory Structure

The input directory contains a collection of skill directories. Each skill is contained within its own directory and must have a `SKILL.md` file.

```
input-directory/
├── skill-name-1/
│   ├── SKILL.md          # Required
│   ├── scripts/          # Optional: Executable code
│   ├── references/       # Optional: Documentation
│   └── assets/           # Optional: Static resources
└── skill-name-2/
    └── SKILL.md
```

## SKILL.md Format

The `SKILL.md` file consists of YAML frontmatter followed by a Markdown body.

### Frontmatter

The YAML frontmatter contains metadata about the skill.

```yaml
---
name: skill-name
description: A description of what this skill does.
license: MIT
compatibility: Optional compatibility string
metadata:
  key: value
allowed-tools: tool1 tool2
---
```

#### Fields

| Field | Required | Description | Constraints |
|---|---|---|---|
| `name` | Yes | The name of the skill. | 1-64 chars, lowercase alphanumeric and hyphens. Must match parent directory name. |
| `description` | Yes | Description of the skill. | 1-1024 chars. Non-empty. |
| `license` | No | License information. | |
| `compatibility` | No | Environment requirements. | Max 500 chars. |
| `metadata` | No | Custom key-value pairs. | Map of string to string. |
| `allowed-tools` | No | Pre-approved tools. | Space-delimited list. |

### Body

The body of the `SKILL.md` contains the instructions for the agent. It can include:
- Step-by-step instructions
- Examples
- References to other files (scripts, docs) using relative paths.

## Validation Rules

1.  **Existence**: `SKILL.md` must exist in each skill directory.
2.  **Frontmatter**: Must be valid YAML and contain required fields (`name`, `description`).
3.  **Naming**: `name` must match the directory name and follow the regex `^[a-z0-9]+(-[a-z0-9]+)*$`.
4.  **Lengths**: Field constraints (e.g., max 64 chars for name) must be respected.
