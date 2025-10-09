---
title: Automerging GitHub PR's from the exercism.org sync bot
categories: code
date: 2025-10-09 10:57
---

[Exercism.org](https://exercism.org/) has a bot to synchronize your exercise solutions to a GitHub repository. If you're not an Exercism Insider, the bot will only open PR's for you and not merge directly to your primary branch.

To solve this, I got [Copilot](https://github.com/copilot) to create a workflow that automatically merges these PR's. It was interesting to see Copilot request reviews from Copilot, and then addressing the comments.

Here's the workflow, if you want to use it:

```yaml
name: Auto-merge Exercism Solutions

on:
  pull_request:
    types: [opened, synchronize, reopened]

jobs:
  auto-merge:
    runs-on: ubuntu-latest
    if: |
      github.event.pull_request.user.login == 'exercism-solutions-syncer[bot]' &&
      github.event.pull_request.user.type == 'Bot'
    
    permissions:
      contents: write
      pull-requests: write
    
    steps:
      - name: Auto-merge PR
        run: |
          gh pr merge "${{ github.event.pull_request.number }}" --auto --squash
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          GH_REPO: ${{ github.repository }}
```
