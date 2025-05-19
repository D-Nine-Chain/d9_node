# Local Workflow Testing with act

This guide helps you test the GitHub Actions workflows locally using `act`.

## Prerequisites

- Docker installed and running
- act installed (`brew install act`)

## Quick Start

```bash
# Test the entire workflow (push event)
act

# Test specific job
act -j build

# Test with specific event (tag push for release)
act push --tag v1.0.0
```

## Common Test Commands

### 1. Test Build Job
```bash
# Test just the build job
act -j build

# Test build with secrets
act -j build --secret-file .secrets.act

# Test specific architecture
act -j build --matrix target:x86_64-unknown-linux-gnu
```

### 2. Test Release Workflow
```bash
# Create event file for tag push
echo '{"ref": "refs/tags/v1.0.0", "ref_type": "tag"}' > event.json

# Test release workflow
act push -e event.json -j release

# Test full workflow with tag
act push --tag v1.0.0
```

### 3. Test Notifications
```bash
# Test notification job with secrets
act -j notify --secret-file .secrets.act

# Force notification job to run
act -j notify --secret-file .secrets.act -e <(echo '{"needs": {"build": {"result": "success"}}}')
```

## Debugging

### Verbose Output
```bash
act -v  # Verbose output
act -vv # Very verbose output
```

### List Available Jobs/Events
```bash
act -l  # List all jobs
act -n  # Dry run (show what would run)
```

### Shell into Container
```bash
act -j build --container-options "-it" --entrypoint /bin/bash
```

## Common Issues

1. **Docker not running**: Make sure Docker Desktop is running
2. **Large downloads**: First run might download Ubuntu images
3. **Missing secrets**: Use `--secret-file .secrets.act`
4. **Platform issues**: Use `--platform linux/amd64` on M1 Macs

## Full Workflow Test

```bash
# Test complete workflow as if pushing a tag
act push --tag v1.0.2 --secret-file .secrets.act
```

## Testing Matrix Builds

```bash
# Test all matrix combinations
act -j build

# Test specific matrix combination
act -j build --matrix arch:aarch64
```

## Custom Event Testing

Create `event.json`:
```json
{
  "ref": "refs/tags/v1.0.3",
  "repository": {
    "name": "d9_node",
    "owner": {
      "login": "D-Nine-Chain"
    }
  },
  "head_commit": {
    "message": "Test commit"
  }
}
```

Then run:
```bash
act push -e event.json
```

## Tips

1. Use `-P ubuntu-latest=catthehacker/ubuntu:full-latest` for better compatibility
2. Add `--pull=false` to skip pulling images if already downloaded
3. Use `--artifact-server-path /tmp/artifacts` to test artifact uploads

## Cleaning Up

```bash
# Remove event files
rm -f event.json

# Prune Docker images if needed
docker system prune -a