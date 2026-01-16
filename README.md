# Agent Knowledge Manager

> **Note:** This is primarily a personal project with no active maintenance. See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

A Rust application designed to manage and organize agent skills/capabilities for various AI tools.

## Overview

This tool acts as a central manager for agent skills. It reads a set of skill definition files from a source directory, validates them, and processes them to generate outputs formatted for specific AI tools and documentation.

## Features

- **Skill Processing**: Recursively traverses input directories to find agent skill definitions.
- **Validation**: Enforces strict naming conventions and structure (e.g., ALL CAPS filenames, matching directory names).
- **Documentation Generation**: Generates an `AGENTS.md` file (in XML format) containing a summary of all available tools and skills.
- **Organization**: Outputs skills in a structured directory format.

## Usage

The CLI tool `persona` supports several commands.

```bash
persona [GLOBAL_OPTIONS] <COMMAND>
```

### Global Options

- `-i, --input <DIR_OR_GLOB>`: Path to input directories. Can be specified multiple times. Defaults to `.agent`.
- `-v, --verbose`: Increase verbosity level.

### Commands

#### Default (Build)

Running without a subcommand processes inputs and generates the summary.

```bash
persona [GLOBAL_OPTIONS] [--output <DIR>]
```

- `-o, --output <DIR>`: Optional path to generate organized file structure.

#### List

Lists parsed files organized by category.

```bash
persona list
```

#### Check

Runs validation on the inputs without generating output. Ideal for CI/CD.

```bash
persona check
```

## Input Format

Input entities are defined in Markdown files with YAML frontmatter within a specific directory structure.

- Filenames must be **ALL CAPS** (e.g., `SKILL.md`).
- Directory names define the category/subcategory.
- Frontmatter must contain at least `name` and `description`.

See [.specs/01_input_format.md](.specs/01_input_format.md) for full details.

## Output

- **`AGENTS.md`**: An XML summary of the agent capabilities generated in the root.
- **Output Directory**: If specified, a mirrored structure of the input with processed files.

See [.specs/03_output_generation.md](.specs/03_output_generation.md) for full details.
