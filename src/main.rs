//! FGP GitHub Daemon
//!
//! Fast daemon for GitHub operations using the gh CLI for authentication.
//!
//! # Methods
//! - `repos` - List your repositories
//! - `issues` - List issues for a repository
//! - `notifications` - Get unread notifications
//! - `pr_status` - Check PR status
//!
//! # Run
//! ```bash
//! cargo run --release
//! ```
//!
//! # Test
//! ```bash
//! fgp call github.repos -p '{"limit": 5}'
//! ```

use anyhow::{bail, Context, Result};
use fgp_daemon::service::{MethodInfo, ParamInfo};
use fgp_daemon::{FgpServer, FgpService};
use serde_json::Value;
use std::collections::HashMap;
use std::process::Command;

/// GitHub service using gh CLI for API calls.
struct GithubService;

impl FgpService for GithubService {
    fn name(&self) -> &str {
        "github"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn dispatch(&self, method: &str, params: HashMap<String, Value>) -> Result<Value> {
        match method {
            "repos" => self.list_repos(params),
            "issues" => self.list_issues(params),
            "notifications" => self.get_notifications(params),
            "pr_status" => self.pr_status(params),
            "user" => self.get_user(),
            _ => bail!("Unknown method: {}", method),
        }
    }

    fn method_list(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "repos".into(),
                description: "List your repositories".into(),
                params: vec![ParamInfo {
                    name: "limit".into(),
                    param_type: "integer".into(),
                    required: false,
                    default: Some(Value::Number(10.into())),
                }],
            },
            MethodInfo {
                name: "issues".into(),
                description: "List issues for a repository".into(),
                params: vec![
                    ParamInfo {
                        name: "repo".into(),
                        param_type: "string".into(),
                        required: true,
                        default: None,
                    },
                    ParamInfo {
                        name: "state".into(),
                        param_type: "string".into(),
                        required: false,
                        default: Some(Value::String("open".into())),
                    },
                    ParamInfo {
                        name: "limit".into(),
                        param_type: "integer".into(),
                        required: false,
                        default: Some(Value::Number(10.into())),
                    },
                ],
            },
            MethodInfo {
                name: "notifications".into(),
                description: "Get unread notifications".into(),
                params: vec![],
            },
            MethodInfo {
                name: "pr_status".into(),
                description: "Check PR status for current branch".into(),
                params: vec![ParamInfo {
                    name: "repo".into(),
                    param_type: "string".into(),
                    required: false,
                    default: None,
                }],
            },
            MethodInfo {
                name: "user".into(),
                description: "Get current authenticated user".into(),
                params: vec![],
            },
        ]
    }

    fn on_start(&self) -> Result<()> {
        // Verify gh CLI is authenticated
        let output = Command::new("gh")
            .args(["auth", "status"])
            .output()
            .context("Failed to run gh CLI - is it installed?")?;

        if !output.status.success() {
            bail!(
                "gh CLI not authenticated. Run 'gh auth login' first.\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        tracing::info!("GitHub daemon starting - gh CLI authenticated");
        Ok(())
    }
}

impl GithubService {
    /// List repositories.
    fn list_repos(&self, params: HashMap<String, Value>) -> Result<Value> {
        let limit = params
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(10);

        let output = Command::new("gh")
            .args([
                "repo",
                "list",
                "--json",
                "name,owner,description,isPrivate,updatedAt,url",
                "--limit",
                &limit.to_string(),
            ])
            .output()
            .context("Failed to run gh repo list")?;

        if !output.status.success() {
            bail!("gh repo list failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        let repos: Value = serde_json::from_slice(&output.stdout)
            .context("Failed to parse gh output")?;

        Ok(serde_json::json!({
            "repos": repos,
            "count": repos.as_array().map(|a| a.len()).unwrap_or(0)
        }))
    }

    /// List issues for a repository.
    fn list_issues(&self, params: HashMap<String, Value>) -> Result<Value> {
        let repo = params
            .get("repo")
            .and_then(|v| v.as_str())
            .context("repo parameter is required")?;

        let state = params
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("open");

        let limit = params
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(10);

        let output = Command::new("gh")
            .args([
                "issue",
                "list",
                "--repo",
                repo,
                "--state",
                state,
                "--json",
                "number,title,author,state,createdAt,url",
                "--limit",
                &limit.to_string(),
            ])
            .output()
            .context("Failed to run gh issue list")?;

        if !output.status.success() {
            bail!("gh issue list failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        let issues: Value = serde_json::from_slice(&output.stdout)
            .context("Failed to parse gh output")?;

        Ok(serde_json::json!({
            "repo": repo,
            "state": state,
            "issues": issues,
            "count": issues.as_array().map(|a| a.len()).unwrap_or(0)
        }))
    }

    /// Get unread notifications.
    fn get_notifications(&self, _params: HashMap<String, Value>) -> Result<Value> {
        let output = Command::new("gh")
            .args([
                "api",
                "/notifications",
                "-q",
                ".[] | {id, unread, reason, subject: .subject.title, repo: .repository.full_name, url: .subject.url}",
            ])
            .output()
            .context("Failed to run gh api")?;

        if !output.status.success() {
            bail!("gh api failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        // Parse JSONL output
        let stdout = String::from_utf8_lossy(&output.stdout);
        let notifications: Vec<Value> = stdout
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        Ok(serde_json::json!({
            "notifications": notifications,
            "unread_count": notifications.len()
        }))
    }

    /// Check PR status.
    fn pr_status(&self, params: HashMap<String, Value>) -> Result<Value> {
        let mut args = vec!["pr", "status", "--json", "currentBranch,createdBy,reviews,statusCheckRollup"];

        let repo;
        if let Some(r) = params.get("repo").and_then(|v| v.as_str()) {
            repo = r.to_string();
            args.extend(["--repo", &repo]);
        }

        let output = Command::new("gh")
            .args(&args)
            .output()
            .context("Failed to run gh pr status")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Not being in a repo is not an error, just return empty
            if stderr.contains("not a git repository") {
                return Ok(serde_json::json!({
                    "error": "Not in a git repository",
                    "has_pr": false
                }));
            }
            bail!("gh pr status failed: {}", stderr);
        }

        let status: Value = serde_json::from_slice(&output.stdout)
            .context("Failed to parse gh output")?;

        Ok(status)
    }

    /// Get current authenticated user.
    fn get_user(&self) -> Result<Value> {
        let output = Command::new("gh")
            .args(["api", "/user"])
            .output()
            .context("Failed to run gh api /user")?;

        if !output.status.success() {
            bail!("gh api failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        let user: Value = serde_json::from_slice(&output.stdout)
            .context("Failed to parse gh output")?;

        Ok(serde_json::json!({
            "login": user["login"],
            "name": user["name"],
            "email": user["email"],
            "avatar_url": user["avatar_url"],
            "public_repos": user["public_repos"],
            "followers": user["followers"],
            "following": user["following"]
        }))
    }
}

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("fgp_github=debug,fgp_daemon=debug")
        .init();

    println!("Starting GitHub daemon...");
    println!("Socket: ~/.fgp/services/github/daemon.sock");
    println!();
    println!("Test with:");
    println!("  fgp call github.repos -p '{{\"limit\": 5}}'");
    println!("  fgp call github.user");
    println!();

    let server = FgpServer::new(GithubService, "~/.fgp/services/github/daemon.sock")?;
    server.serve()?;

    Ok(())
}
