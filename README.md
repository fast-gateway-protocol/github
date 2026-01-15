# fgp-github

FGP daemon for GitHub operations using the gh CLI for authentication.

Part of the [Fast Gateway Protocol (FGP)](https://github.com/fast-gateway-protocol) ecosystem - the universal package manager for AI agents.

## Installation

### Prerequisites

1. **gh CLI installed and authenticated**:
   ```bash
   brew install gh
   gh auth login
   ```

2. **fgp CLI installed**:
   ```bash
   cargo install fgp
   ```

### Build from Source

```bash
git clone https://github.com/fast-gateway-protocol/github
cd fgp-github
cargo build --release
```

### Install as FGP Package

```bash
fgp install github  # (coming soon)
```

## Usage

### Start the Daemon

```bash
# Manual start
./target/release/fgp-github

# Or via fgp CLI (auto-discovers installed daemons)
fgp start github
```

### Make Calls

```bash
# Get authenticated user
fgp call github.user

# List repositories
fgp call github.repos -p '{"limit": 5}'

# List issues
fgp call github.issues -p '{"repo": "owner/repo", "state": "open"}'

# Check notifications
fgp call github.notifications

# Check PR status
fgp call github.pr_status -p '{"repo": "owner/repo"}'
```

### Check Status

```bash
fgp health github
fgp methods github
```

## Methods

| Method | Description | Required Params |
|--------|-------------|-----------------|
| `repos` | List your repositories | `limit` (optional, default: 10) |
| `issues` | List issues for a repository | `repo` (required), `state` (optional), `limit` (optional) |
| `notifications` | Get unread notifications | none |
| `pr_status` | Check PR status for current branch | `repo` (optional) |
| `user` | Get authenticated user info | none |

## Performance

The FGP daemon architecture provides:
- **10-30ms** response times (after gh CLI overhead)
- Persistent connections via UNIX sockets
- No cold start penalty on subsequent calls
- Shared authentication via gh CLI

Compare to MCP's stdio-based approach which requires process spawn per call (200-500ms).

## Architecture

```
fgp call github.repos
        │
        ▼
    fgp CLI
        │
        ▼
    UNIX Socket
    ~/.fgp/services/github/daemon.sock
        │
        ▼
    fgp-github daemon
        │
        ▼
    gh CLI (authenticated)
        │
        ▼
    GitHub API
```

## Protocol

Uses [FGP Protocol v1](https://github.com/fast-gateway-protocol/protocol/blob/main/FGP-PROTOCOL.md):

Request:
```json
{"id":"uuid","v":1,"method":"github.repos","params":{"limit":5}}
```

Response:
```json
{"id":"uuid","ok":true,"result":{...},"meta":{"server_ms":12}}
```

## Development

Built with [daemon](https://github.com/fast-gateway-protocol/daemon) Rust SDK:

```rust
use fgp_daemon::{FgpServer, FgpService};

struct GithubService;

impl FgpService for GithubService {
    fn name(&self) -> &str { "github" }
    fn version(&self) -> &str { "1.0.0" }
    fn dispatch(&self, method: &str, params: HashMap<String, Value>) -> Result<Value> {
        // Handle methods...
    }
}
```

## Troubleshooting

### gh CLI Not Authenticated

**Symptom:** Requests fail with authentication errors

**Solution:**
```bash
# Check auth status
gh auth status

# Re-authenticate if needed
gh auth login
```

### Permission Denied

**Symptom:** "Resource not accessible" or 403 errors

**Check:**
1. Your token has required scopes: `gh auth status`
2. You have access to the repository
3. For private repos, ensure `repo` scope is granted

### Rate Limiting

**Symptom:** 429 errors or "rate limit exceeded"

**Solutions:**
1. GitHub has 5000 requests/hour for authenticated users
2. Check remaining: `gh api rate_limit`
3. Wait for reset or reduce request frequency

### Slow Responses

**Symptom:** Calls take longer than expected

**Check:**
1. gh CLI overhead is ~100-200ms per call
2. First call may be slower (token validation)
3. For bulk operations, consider batching

### Empty Notifications

**Symptom:** `notifications` returns empty when you have unread

**Note:** GitHub notifications can be complex:
1. Check web interface for comparison
2. Some notifications may be filtered by type
3. Use `gh api notifications` to debug

### Connection Refused

**Symptom:** "Connection refused" when calling daemon

**Solution:**
```bash
# Check if daemon is running
pgrep -f fgp-github

# Restart daemon
fgp stop github
fgp start github

# Verify gh is working
gh api user
```

## License

MIT
