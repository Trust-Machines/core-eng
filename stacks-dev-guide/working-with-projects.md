- Read about the new (as of 2023) GitHub Projects: https://github.blog/2022-11-15-the-journey-of-your-work-has-never-been-clearer/

- GitHub Projects are a collection of issues are PRs.

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

- The following searchers should be empty. Else, please fix the missing meta data:
  - [core-eng Issues with no linked Project](https://github.com/Trust-Machines/core-eng/issues?q=is%3Aissue+is%3Aopen+no%3Aproject)
  - [core-eng Pulls with ono linked Issues](https://github.com/Trust-Machines/core-eng/pulls?q=is%3Apr+is%3Aopen+-linked%3Aissue+)
