---
name: jj-staging
description: Use the jj command line tool for staging work, ensuring commits are created on top of main and at key parts of the task plan when tests pass.
---

# Jujutsu (jj) Staging

This skill enables the agent to use `jj` (Jujutsu) for version control and staging work.

## Requirements

This skill should only activate if the `jj` tool is present.

## Instructions

When using this skill, adhere to the following rules:

1.  **Always create a new commit on top of the main git branch.**
    -   Ensure you are working off the latest `main`.
    -   Use `jj new main` to start a new change on top of main.

2.  **Create new commits at key parts of the task plan.**
    -   Do not squash everything into one commit at the end unless instructed otherwise.
    -   Create granular commits that represent logical steps in the plan.
    -   After completing a step and verifying it, use `jj describe` to give it a message, then `jj new` to start the next step on top of it.

3.  **Commits should only be created when a task is completed and all tests pass.**
    -   Before "committing" (describing and moving to a new revision), ensure that the code compiles and relevant tests pass.
    -   Do not leave broken code in a described commit if possible.

## Usage Examples

### Starting a new task

Start a new revision on top of main:

```bash
jj new main
```

### Staging work (Committing)

When a sub-task is complete and verified:

1.  Describe the current change (equivalent to committing):

    ```bash
    jj describe -m "feat: implement step 1 of the plan"
    ```

2.  Create a new empty revision on top of it for the next task:

    ```bash
    jj new
    ```
