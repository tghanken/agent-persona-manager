# CLI Interface Specification

This document defines the Command Line Interface (CLI) for the Agent Knowledge Manager (`persona`).

## Usage

```bash
persona [OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

## Global Options

These options apply to the application execution environment.

-   `-i, --input <DIR_OR_GLOB>`: Path to input directories. Can be specified multiple times. Supports globs.
-   `-v, --verbose`: Increase verbosity level (e.g., `-v`, `-vv`, `-vvv`) to change tracing subscriber format.
-   `-V, --version`: Print version.
-   `-h, --help`: Print help.

## Commands

### Default (Build/Generate)

Running the application without a subcommand (or with a specific `build` command) processes the inputs and generates the summary.

**Usage:**
```bash
persona [GLOBAL_OPTIONS] [--output <DIR>]
```

**Options:**
-   `-o, --output <DIR>`: Optional. Path to the directory where the full organized set of files will be generated. If omitted, only the `AGENTS.md` file is generated.

**Behavior:**
1.  Reads and parses all inputs specified by global flags.
2.  Validates all entities. Fails if any error is found.
3.  Generates `AGENTS.md` in the repository root (current working directory).
4.  If `--output` is specified, generates the organized directory structure in the target directory.

### List

Prints a list of parsed files, organized by category.

**Usage:**
```bash
persona [GLOBAL_OPTIONS] list
```

**Behavior:**
1.  Reads and parses all inputs specified by global flags.
2.  Validates all entities.
3.  Prints the hierarchy of detected categories and entities to stdout.

### Check

Runs validation on the inputs without generating output.

**Usage:**
```bash
persona [GLOBAL_OPTIONS] check
```

**Behavior:**
1.  Reads and parses all inputs specified by global flags.
2.  Validates the structure and content against the [Input Format Specification](./01_input_format.md).
3.  Exits with status code 0 if valid, non-zero if invalid.
4.  Prints validation errors to stderr.

## Exit Codes

-   `0`: Success / Valid.
-   `1`: Error (Validation failed, file I/O error, etc.).
