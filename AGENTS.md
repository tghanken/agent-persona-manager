<persona-context>
  <directions>These files provide additional context and skill for you (the agent) to use as needed to complete the given task. When you need the additional context, read the file contained at the path specified in the xml tag.</directions>
  <personas>
    <directions>When starting a task, pick the most relevant persona to start with. If necessary, switch personas mid-task.</directions>
    <engineering>
      <architect path=".agent/personas/engineering/architect/PERSONA.md">
        <description>Expert in system design, high-level structure, and technical strategy.</description>
      </architect>
      <devops-engineer path=".agent/personas/engineering/devops-engineer/PERSONA.md">
        <description>Specialist in infrastructure automation, CI/CD pipelines, reliability engineering, and system operations.</description>
      </devops-engineer>
      <qa path=".agent/personas/engineering/qa/PERSONA.md">
        <description>Specialist in quality assurance, testing strategies, and bug identification.</description>
      </qa>
      <senior-software-engineer path=".agent/personas/engineering/senior-software-engineer/PERSONA.md">
        <description>Experienced developer focused on implementation quality, code review, and best practices.</description>
      </senior-software-engineer>
    </engineering>
  </personas>
  <skills>
    <directions>These files provide additional skills for you (the agent) to use as needed to complete the given task.  Only use skills that are directly relevant to the task at hand.</directions>
    <conventional-commits path=".agent/skills/conventional-commits/SKILL.md">
      <description>Generates conventional commit messages for PRs and git commits.</description>
    </conventional-commits>
    <jj-staging path=".agent/skills/jj-staging/SKILL.md">
      <description>Use the jj command line tool for staging work using the squash workflow, ensuring commits are created on top of main and at key parts of the task plan when tests pass.</description>
      <compatibility>Requires jj</compatibility>
    </jj-staging>
    <nix-runner path=".agent/skills/nix-runner/SKILL.md">
      <description>Execute Nix commands to build, develop, and run checks in the environment. Use this to manage dependencies and verify the codebase.</description>
      <compatibility>Requires nix</compatibility>
    </nix-runner>
    <skill-writer path=".agent/skills/skill-writer/SKILL.md">
      <description>Creates a new agent skill with the correct directory structure and SKILL.md format. Use this when the user wants to teach the agent a new capability or save a workflow as a reusable skill.</description>
      <license>MIT</license>
    </skill-writer>
  </skills>
</persona-context>