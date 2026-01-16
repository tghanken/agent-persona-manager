---
name: jj-staging
description: Use the jj command line tool for staging work using the squash workflow, ensuring commits are created on top of main and at key parts of the task plan when tests pass.
compatibility: Requires jj
---

# Jujutsu (jj) Staging

This skill enables the agent to use `jj` (Jujutsu) for version control and staging work using the **Squash Workflow**.

## Requirements

This skill should only activate if the `jj` tool is present.

## Instructions

When using this skill, adhere to the following rules:

1.  **Always create a new commit on top of the main git branch.**
    -   Ensure you are working off the latest `main`.
    -   Use `jj new main` to start a new change on top of main.

2.  **Use the Squash Workflow.**
    -   **Describe first**: Give the commit you are building a meaningful description.
    -   **Work in a scratch commit**: Create a new empty change on top of your described commit.
    -   **Squash to save**: As you complete work, use `jj squash` to move changes from your scratch commit into the parent (described) commit.

3.  **Create new commits at key parts of the task plan.**
    -   Do not squash everything into one single commit for the entire plan.
    -   Create granular commits that represent logical steps.
    -   Once a step is complete and verified, the scratch commit should be empty (because you squashed). Use this empty scratch commit as the start of the *next* logical step by describing it.

4.  **Commits should only be created when a task is completed and all tests pass.**
    -   Before finalizing a step (moving to the next one), ensure code compiles and tests pass.

## Usage Examples

### Starting a new task (Step 1)

1.  Start a new change on top of main and describe it:
    ```bash
    jj new main -m "feat: implement step 1"
    ```
    *This is now your "Step 1" commit.*

2.  Create a scratch commit on top of it to work in:
    ```bash
    jj new
    ```

### Doing Work (The Loop)

1.  Make changes to files.
2.  Check status:
    ```bash
    jj st
    ```
3.  Squash changes into the parent ("Step 1") commit:
    ```bash
    jj squash
    ```
    *Your working copy (`@`) is now empty again, and changes are safely in the parent.*

### Moving to the Next Task (Step 2)

Once Step 1 is complete and tests pass:

1.  You are currently in an empty scratch commit on top of "Step 1".
2.  Describe this empty commit as "Step 2":
    ```bash
    jj describe -m "feat: implement step 2"
    ```
3.  Create a *new* scratch commit on top of "Step 2" to work in:
    ```bash
    jj new
    ```
