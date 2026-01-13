# GitHub Workflow

Fast GitHub operations via FGP daemon. 10x faster than MCP-based tools.

## Available Methods

| Method | Description |
|--------|-------------|
| `github.repos` | List your repositories |
| `github.issues` | List issues for a repository |
| `github.notifications` | Get unread notifications |
| `github.pr_status` | Check PR status |
| `github.user` | Get authenticated user |

## Commands

### github.repos - List Repositories

**Parameters:**
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `limit` | integer | No | 10 | Maximum repos to return |

```bash
fgp call github.repos -p '{"limit": 10}'
```

**Response:**
```json
{
  "repos": [{"name": "...", "owner": {...}, "description": "..."}],
  "count": 10
}
```

---

### github.issues - List Issues

**Parameters:**
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `repo` | string | Yes | - | Repository (owner/repo format) |
| `state` | string | No | "open" | Issue state: open, closed, all |
| `limit` | integer | No | 10 | Maximum issues to return |

```bash
fgp call github.issues -p '{"repo": "owner/repo", "state": "open", "limit": 10}'
```

---

### github.notifications - Get Notifications

No parameters. Returns unread notifications.

```bash
fgp call github.notifications
```

**Response:**
```json
{
  "notifications": [...],
  "unread_count": 5
}
```

---

### github.pr_status - Check PR Status

**Parameters:**
| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `repo` | string | No | - | Repository (uses current dir if not specified) |

```bash
fgp call github.pr_status -p '{"repo": "owner/repo"}'
```

---

### github.user - Get User Info

No parameters. Returns authenticated user.

```bash
fgp call github.user
```

**Response:**
```json
{
  "login": "username",
  "name": "Full Name",
  "email": "user@example.com",
  "public_repos": 42
}
```

## Workflow Steps

1. **User requests GitHub action**
2. **Run appropriate `fgp call github.*` command**
3. **Parse JSON response**
4. **Present results to user**

## Troubleshooting

| Issue | Check | Fix |
|-------|-------|-----|
| Auth failed | `gh auth status` | `gh auth login` |
| Daemon not running | `fgp status github` | `fgp start github` |
| Permission denied | gh CLI scopes | Re-auth with needed scopes |

## Performance

- Cold start: ~50ms
- Warm call: ~10-30ms
