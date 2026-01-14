import os
import sys
import re


def validate_skill_name(name):
    return re.match(r"^[a-z0-9]+(-[a-z0-9]+)*$", name) is not None and len(name) <= 64


def validate_skill_structure(skill_path):
    skill_name = os.path.basename(skill_path)
    print(f"Validating skill: {skill_name}...")

    if not validate_skill_name(skill_name):
        print(f"  [ERROR] Invalid skill name: {skill_name}")
        return False

    skill_md_path = os.path.join(skill_path, "SKILL.md")
    if not os.path.exists(skill_md_path):
        print(f"  [ERROR] Missing SKILL.md in {skill_path}")
        return False

    # Basic frontmatter validation
    with open(skill_md_path, "r") as f:
        content = f.read()

    if not content.startswith("---"):
        print(f"  [ERROR] SKILL.md must start with frontmatter (---)")
        return False

    # Simple check for required fields in frontmatter
    # A robust parser would be better, but regex is sufficient for this check
    frontmatter_match = re.match(r"^---\n(.*?)\n---", content, re.DOTALL)
    if not frontmatter_match:
        print(f"  [ERROR] Malformed frontmatter in SKILL.md")
        return False

    frontmatter = frontmatter_match.group(1)
    if "name:" not in frontmatter:
        print(f"  [ERROR] Missing 'name' in frontmatter")
        return False
    if "description:" not in frontmatter:
        print(f"  [ERROR] Missing 'description' in frontmatter")
        return False

    # Check if name in frontmatter matches directory name
    name_match = re.search(r"name:\s*([a-z0-9-]+)", frontmatter)
    if name_match:
        declared_name = name_match.group(1).strip()
        if declared_name != skill_name:
            print(
                f"  [ERROR] Name in SKILL.md ({declared_name}) does not match directory name ({skill_name})"
            )
            return False

    print(f"  [OK] {skill_name} is valid.")
    return True


def main():
    skills_dir = ".agent/skills"
    if not os.path.exists(skills_dir):
        print(f"Skills directory '{skills_dir}' not found. Skipping validation.")
        return 0

    success = True
    for item in os.listdir(skills_dir):
        item_path = os.path.join(skills_dir, item)
        if os.path.isdir(item_path):
            if not validate_skill_structure(item_path):
                success = False

    if success:
        print("All skills passed validation.")
        return 0
    else:
        print("Some skills failed validation.")
        return 1


if __name__ == "__main__":
    sys.exit(main())
