# CLI Interface Specification

This document defines the Command Line Interface (CLI) for the Agent Knowledge Manager (`persona`).

## Usage

```bash
persona [COMMAND] [OPTIONS]
```

## Commands

### Default (Build/Generate)

Running the application without a subcommand (or with a specific build command if implemented) processes the skills and generates the output.

**Usage:**
```bash
persona [OPTIONS] --input <INPUT_DIR> --output <OUTPUT_DIR>
```

**Options:**
- `-i, --input <DIR>`: Path to the directory containing agent skill definitions. Defaults to current directory or specific location if configured.
- `-o, --output <DIR>`: Path to the directory where the output will be generated.
- `-h, --help`: Print help.
- `-V, --version`: Print version.

**Behavior:**
1.  Reads skill definitions from the input directory.
2.  Validates the skills.
3.  Generates the organized skill directories in the output directory.
4.  Generates `AGENTS.md` in the output directory (or root).

### Check

Runs the validation logic on the input directory without generating output. Suitable for CI environments.

**Usage:**
```bash
persona check --input <INPUT_DIR>
```

**Options:**
- `-i, --input <DIR>`: Path to the directory containing agent skill definitions.

**Behavior:**
1.  Reads skill definitions from the input directory.
2.  Validates the skills against the [Input Format Specification](./01_input_format.md).
3.  Exits with status code 0 if valid, non-zero if invalid.
4.  Prints validation errors to stderr.

## Exit Codes

- `0`: Success / Valid.
- `1`: Error (Validation failed, file I/O error, etc.).
