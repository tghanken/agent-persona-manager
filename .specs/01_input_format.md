# Input Format Specification

This document specifies the expected format for agent knowledge entities (Skills, Personas, Rules, etc.) provided as input to the Agent Knowledge Manager.

## Directory Structure

The input organization relies on directory structure to define categories and subcategories. The tool traverses the input directories to find entity definitions.

```
input-directory/
├── category/               # e.g., "skills", "personas"
│   ├── subcategory/        # Optional: e.g., "coding", "finance" (can be nested)
│   │   ├── entity-name/
│   │   │   ├── DEFINITION.md   # Required (e.g., SKILL.md, PERSONA.md)
│   │   │   ├── scripts/        # Optional
│   │   │   └── ...
│   │   └── entity-name-2/
│   │       └── ...
│   └── entity-name-3/
│       └── ...
└── ...
```

## Definition File Format

Each entity directory must contain a Markdown file with YAML frontmatter. The filename conventions (e.g., `SKILL.md`, `PERSONA.md`) may vary by category, but the format is consistent.

### Frontmatter

The YAML frontmatter contains metadata about the entity. All fields present in the frontmatter will be parsed and included in the output.

```yaml
---
name: entity-name
description: A description of what this entity represents.
# ... other fields as needed (e.g., license, compatibility, etc.)
---
```

#### Common Fields

| Field | Required | Description | Constraints |
|---|---|---|---|
| `name` | Yes | The name of the entity. | 1-64 chars, lowercase alphanumeric and hyphens. Must match parent directory name. |
| `description` | Yes | Description of the entity. | Non-empty string. |

### Body

The body of the markdown file contains the content/instructions for the entity.

## Validation Rules

1.  **Structure**: Entities must be contained in their own directory matching their `name`.
2.  **Existence**: A valid definition markdown file must exist in the entity directory.
3.  **Frontmatter**: Must be valid YAML and contain required fields (`name`, `description`).
4.  **Consistency**: The `name` field must match the parent directory name.
5.  **Strict Mode**: Any parsing error or validation failure in the scanned directories causes the process to fail.
