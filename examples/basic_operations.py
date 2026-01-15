#!/usr/bin/env python3
"""
GitHub Daemon - Basic Operations Example

Demonstrates common GitHub operations using the FGP GitHub daemon.
Requires:
  - GitHub daemon running (`fgp start github`)
  - GitHub CLI authenticated (`gh auth login`)
"""

import json
import socket
import uuid
from pathlib import Path

SOCKET_PATH = Path.home() / ".fgp/services/github/daemon.sock"


def call_daemon(method: str, params: dict = None) -> dict:
    """Send a request to the GitHub daemon and return the response."""
    request = {
        "id": str(uuid.uuid4()),
        "v": 1,
        "method": method,
        "params": params or {}
    }

    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as sock:
        sock.connect(str(SOCKET_PATH))
        sock.sendall((json.dumps(request) + "\n").encode())

        response = b""
        while True:
            chunk = sock.recv(4096)
            if not chunk:
                break
            response += chunk
            if b"\n" in response:
                break

        return json.loads(response.decode().strip())


def list_repos(user: str = None, limit: int = 10):
    """List repositories for a user or the authenticated user."""
    target = user or "authenticated user"
    print(f"\n📚 Repositories for {target}")
    print("-" * 40)

    params = {"limit": limit}
    if user:
        params["user"] = user

    result = call_daemon("github.repos", params)

    if result.get("ok"):
        repos = result["result"].get("repos", [])
        for repo in repos:
            stars = repo.get("stargazers_count", 0)
            print(f"  • {repo.get('full_name')}")
            print(f"    ⭐ {stars} stars | {repo.get('language', 'Unknown')}")
            if repo.get("description"):
                print(f"    {repo['description'][:60]}...")
            print()
    else:
        print(f"  ❌ Error: {result.get('error')}")


def list_issues(repo: str, state: str = "open", limit: int = 10):
    """List issues for a repository."""
    print(f"\n🐛 Issues for {repo} ({state})")
    print("-" * 40)

    result = call_daemon("github.issues", {
        "repo": repo,
        "state": state,
        "limit": limit
    })

    if result.get("ok"):
        issues = result["result"].get("issues", [])
        if not issues:
            print(f"  No {state} issues found")
        for issue in issues:
            labels = ", ".join(l.get("name", "") for l in issue.get("labels", []))
            print(f"  #{issue.get('number')} {issue.get('title')}")
            if labels:
                print(f"    Labels: {labels}")
            print(f"    Author: {issue.get('user', {}).get('login', 'unknown')}")
            print()
    else:
        print(f"  ❌ Error: {result.get('error')}")


def list_prs(repo: str, state: str = "open", limit: int = 10):
    """List pull requests for a repository."""
    print(f"\n🔀 Pull Requests for {repo} ({state})")
    print("-" * 40)

    result = call_daemon("github.prs", {
        "repo": repo,
        "state": state,
        "limit": limit
    })

    if result.get("ok"):
        prs = result["result"].get("prs", [])
        if not prs:
            print(f"  No {state} pull requests found")
        for pr in prs:
            print(f"  #{pr.get('number')} {pr.get('title')}")
            print(f"    Author: {pr.get('user', {}).get('login', 'unknown')}")
            print(f"    Branch: {pr.get('head', {}).get('ref', 'unknown')}")
            print()
    else:
        print(f"  ❌ Error: {result.get('error')}")


def get_notifications(limit: int = 10):
    """Get recent notifications."""
    print(f"\n🔔 Recent Notifications")
    print("-" * 40)

    result = call_daemon("github.notifications", {"limit": limit})

    if result.get("ok"):
        notifications = result["result"].get("notifications", [])
        if not notifications:
            print("  No unread notifications")
        for notif in notifications:
            print(f"  • {notif.get('subject', {}).get('title', '(no title)')}")
            print(f"    Repo: {notif.get('repository', {}).get('full_name', 'unknown')}")
            print(f"    Type: {notif.get('subject', {}).get('type', 'unknown')}")
            print()
    else:
        print(f"  ❌ Error: {result.get('error')}")


def create_issue(repo: str, title: str, body: str = None, labels: list = None):
    """Create a new issue."""
    print(f"\n➕ Creating issue in {repo}")

    params = {
        "repo": repo,
        "title": title
    }
    if body:
        params["body"] = body
    if labels:
        params["labels"] = labels

    result = call_daemon("github.create_issue", params)

    if result.get("ok"):
        issue_num = result["result"].get("number")
        url = result["result"].get("html_url")
        print(f"  ✅ Issue #{issue_num} created!")
        print(f"  URL: {url}")
    else:
        print(f"  ❌ Error: {result.get('error')}")


if __name__ == "__main__":
    print("GitHub Daemon Examples")
    print("=" * 40)

    # Check daemon health first
    health = call_daemon("health")
    if not health.get("ok"):
        print("❌ GitHub daemon not running. Start with: fgp start github")
        exit(1)

    print("✅ GitHub daemon is healthy")

    # Run examples - use a public repo for testing
    list_repos(limit=5)
    list_issues("fast-gateway-protocol/browser", state="open", limit=5)
    list_prs("fast-gateway-protocol/browser", state="open", limit=5)
    get_notifications(limit=5)

    # Uncomment to create a test issue:
    # create_issue(
    #     repo="your-username/test-repo",
    #     title="Test issue from FGP",
    #     body="This issue was created via the FGP GitHub daemon.",
    #     labels=["test"]
    # )
