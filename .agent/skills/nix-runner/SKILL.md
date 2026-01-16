---
name: nix-runner
description: Execute Nix commands to build, develop, and run checks in the environment. Use this to manage dependencies and verify the codebase.
license: MIT
---

# Nix Runner

This skill allows the agent to interact with the Nix environment.

## Commands

### Build
Build the project or a specific attribute:
```bash
nix build .#<attribute>
```

### Develop
Enter a development shell or run a command within it:
```bash
nix develop -c <command>
```

### Check
Run flake checks:
```bash
nix flake check
```

## Usage

Use this skill when you need to:
- Verify that the Nix configuration is correct.
- Build artifacts defined in the flake.
- Run tests or checks defined in the flake.
