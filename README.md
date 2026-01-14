# Agent Knowledge Manager

A Rust application designed to manage and organize agent skills/capabilities for various AI tools.

## Overview

This tool acts as a central manager for agent skills. It reads a set of skill definition files from a source directory and processes them to generate outputs formatted for specific AI tools and documentation.

## Features

- **Skill Processing**: Takes a directory of agent skill files as input.
- **Multi-Format Output**: Outputs skills in the desired directory patterns/formats required by different AI tools.
- **Documentation Generation**: Generates an `AGENTS.md` file (in XML format) containing a summary of all available tools and skills.
- **CI Validation**: Includes a strict `check` mode designed for Continuous Integration (CI) environments to validate all skill files and ensure consistency.

## Usage

### Running the Application

To run the application and generate outputs:

```bash
cargo run -- [OPTIONS]
```

### Check Mode (CI)

To run in validation mode (does not write outputs, only checks validity):

```bash
cargo run -- check
```

## Input/Output

- **Input**: A directory containing your agent skill definitions.
- **Output**:
  - Organized directories for target AI tools.
  - `AGENTS.md`: An XML summary of the agent capabilities.
