## Requirements:

1. Language: Write all messages in english
2. Format: Strictly follow the conventional commit format: <type>: <description>
3. Allowed Types:
  - docs: 'Documentation only changes'
  - style: 'Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)'
  - ref: 'A code change that neither fixes a bug nor adds a feature'
  - perf: 'A code change that improves performance'
  - test: 'Adding missing tests or correcting existing tests'
  - build: 'Changes that affect the build system or external dependencies'
  - ci: 'Changes to CI configuration files, scripts'
  - chore: 'Other changes that don't modify src or test files'
  - revert: 'Reverts a previous commit'
  - feat: 'A new feature'
  - fix: 'A bug fix'

## Guidelines:
- Subject line: Max 50 characters, imperative mood, no period, begin with lowercase
- Analyze the diff to understand:
  * What files were changed
  * What functionality was added, modified, or removed
  * The impact of changes
- For the commit type, choose based on:
  * feat: New functionality or feature
  * fix: Bug fixes or error corrections
  * ref: Code restructuring without changing functionality
  * docs: Documentation changes only
  * style: Formatting, missing semi-colons, etc
  * test: Adding or modifying tests
  * chore: Maintenance tasks, dependency updates
  * perf: Performance improvements
  * build: Build system or external dependency changes
  * ci: CI configuration changes
- Body (when needed):
  * Explain the motivation for the change
  * Compare previous behavior with new behavior
  * Note any breaking changes or important details
- Footer: Include references to issues, breaking changes if applicable

## Analysis Approach:
1. Identify the primary purpose of the changes
2. Group related changes together
3. Determine the most appropriate type
4. Write a clear, concise subject line
5. Add body details for complex changes

Remember: The commit message should help future developers understand WHY this change was made, not just WHAT was changed.
Here is the git diff to analyze
