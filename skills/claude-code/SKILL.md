---
name: github-fgp
description: Fast GitHub operations via FGP daemon (10x faster than MCP)
tools: ["Bash"]
triggers:
  - "github repos"
  - "list my repositories"
  - "github issues"
  - "github notifications"
  - "check PR status"
  - "github user"
---

# GitHub FGP Skill

Fast GitHub operations using the FGP daemon protocol. 10-30ms response times via persistent UNIX sockets.

## Prerequisites

1. **gh CLI authenticated**: Run `gh auth login` if not already set up
2. **FGP daemon running**: `fgp start github` or daemon auto-starts on first call

## Available Methods

### List Repositories

```bash
fgp call github.repos -p '{"limit": 10}'
```

Returns your repositories with name, owner, description, privacy status, and URL.

### List Issues

```bash
fgp call github.issues -p '{"repo": "owner/repo", "state": "open", "limit": 10}'
```

Parameters:
- `repo` (required): Repository in "owner/repo" format
- `state` (optional): "open", "closed", or "all" (default: "open")
- `limit` (optional): Max issues to return (default: 10)

### Get Notifications

```bash
fgp call github.notifications
```

Returns unread notifications with reason, subject, and repository.

### Check PR Status

```bash
fgp call github.pr_status -p '{"repo": "owner/repo"}'
```

Check pull request status for the current branch. If no repo specified, uses current directory's git remote.

### Get User Info

```bash
fgp call github.user
```

Returns authenticated user's login, name, email, avatar, and stats.

## Usage Tips

- Use `fgp health github` to check daemon status
- Use `fgp methods github` to see all available methods
- Daemon stays warm between calls for fast response times

## Example Workflow

```bash
# Check who you're authenticated as
fgp call github.user

# List your recent repos
fgp call github.repos -p '{"limit": 5}'

# Check open issues on a repo
fgp call github.issues -p '{"repo": "wolfiesch/fgp", "limit": 20}'

# Check notifications
fgp call github.notifications
```
