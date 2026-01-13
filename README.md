# fgp-github

FGP daemon for GitHub operations using the gh CLI for authentication.

Part of the [Fast Gateway Protocol (FGP)](https://github.com/wolfiesch/fgp) ecosystem - the universal package manager for AI agents.

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
git clone https://github.com/wolfiesch/fgp-github
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

Uses [FGP Protocol v1](https://github.com/wolfiesch/fgp/blob/main/FGP-PROTOCOL.md):

Request:
```json
{"id":"uuid","v":1,"method":"github.repos","params":{"limit":5}}
```

Response:
```json
{"id":"uuid","ok":true,"result":{...},"meta":{"server_ms":12}}
```

## Development

Built with [fgp-daemon](https://github.com/wolfiesch/fgp-daemon) Rust SDK:

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

## License

MIT
