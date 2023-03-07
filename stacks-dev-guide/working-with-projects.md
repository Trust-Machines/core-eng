TLDR: no issues without a linked project and no PRs without a linked issue.

- Read about the new (as of 2023) GitHub Projects: https://github.blog/2022-11-15-the-journey-of-your-work-has-never-been-clearer/

- GitHub Projects are a collection of issues and PRs.

- GitHub Issues have:
  - one or more linked Projects
  - one or more linked Pulls (PRs).
  - one or more sub-tasks or sub-issues (also known as children issues)

- The "Issue Graph" is:
  - the implicit graph defined by
    - nodes: Issues and Pulls, and
    - edges: Issue/Sub-Issue, Issue/Pull relationships and links to Issues and Pulls in the comments,
  - close to a Directed Acyclic Graph
    - we do not want cycles for issue/sub-issue edges,
    - however, back-references to ancestor issues in discussions or comments are allowed.

- Issues should be tagged with a project:
  - [Core Eng Project - sBTC](https://github.com/orgs/Trust-Machines/projects/5)
  - [Core Eng - Public](https://github.com/orgs/Trust-Machines/projects/9/views/1) - Anything related to Core Eng Team and not to sBTC
  - [Core Eng - Private](https://github.com/orgs/Trust-Machines/projects/7) - Anything sensitive (devops, etc)
  
- Pulls should be tagged with an issue, or tagged with a project (if moving fast, but let's minimize this!)

- The following searches should be empty. Else, please fix the missing meta data:
  - [core-eng Issues with no linked Project](https://github.com/Trust-Machines/core-eng/issues?q=is%3Aissue+is%3Aopen+no%3Aproject)
  - [core-eng Pulls with ono linked Issues](https://github.com/Trust-Machines/core-eng/pulls?q=is%3Apr+is%3Aopen+-linked%3Aissue+)
