# D9 Node Installation Guide

This guide covers multiple installation methods for the D9 node, from quick automated installation to manual compilation from source.

[中文安装指南](./installation_zh.md) - Chinese Installation Guide

## System Requirements

- **Operating System**: Ubuntu 22.04 LTS (recommended)
- **Architecture**: x86_64 or ARM64
- **RAM**: Minimum 8GB (16GB recommended)
- **Storage**: Minimum 60GB free space (SSD recommended)
- **Network**: Stable internet connection
- **Ports**: 40100 (P2P), 40200 (RPC), 40300 (WebSocket)

## Quick Install (Recommended)

The fastest way to get a D9 node running is using our automated installation script:

```bash
curl -sSf https://raw.githubusercontent.com/D-Nine-Chain/d9_node/main/scripts/install-d9-node.sh | bash
```

This script will:
1. Verify system requirements
2. Download the latest pre-built binary
3. Configure systemd service
4. Set up validator keys (optional)
5. Start the node automatically

### Script Options

The installation script supports multiple languages:
- English (default)
- Chinese (中文) - [Chinese installation guide](./installation_zh.md)

You'll be prompted to choose your language at the start.

### What the Script Does

1. **System Checks**
   - Verifies Ubuntu 22.04
   - Checks architecture (x86_64/ARM64)
   - Ensures 60GB+ free disk space
   - Configures swap file

2. **Node Installation**
   - Downloads latest release from GitHub
   - Installs to `/usr/local/bin/d9-node`
   - Creates data directory at `/home/ubuntu/node-data`
   - Downloads chain specification

3. **Service Configuration**
   - Creates systemd service file
   - Enables automatic startup
   - Configures logging

4. **Key Management** (Optional)
   - Generates or imports validator keys
   - Sets up keystore
   - Displays node address

## Build from Source

For developers or advanced users who want to compile from source:

### Using Build Script

```bash
curl -sSf https://raw.githubusercontent.com/D-Nine-Chain/d9_node/main/scripts/build-node.sh | bash
```

This script:
1. Installs all dependencies
2. Sets up Rust toolchain
3. Clones the repository
4. Builds the node
5. Configures the service

### Manual Build

#### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
```

#### 2. Install System Dependencies

```bash
sudo apt update
sudo apt install -y build-essential git clang curl libssl-dev llvm \
    libudev-dev make protobuf-compiler pkg-config
```

#### 3. Clone and Build

```bash
git clone https://github.com/D-Nine-Chain/d9_node.git
cd d9_node
cargo build --release
```

Building from source typically takes 15-30 minutes depending on your system.

#### 4. Install Binary

```bash
sudo cp target/release/d9-node /usr/local/bin/
sudo chmod +x /usr/local/bin/d9-node
```

## Manual Service Setup

If you built from source manually, create the systemd service:

1. Create service file:
```bash
sudo nano /etc/systemd/system/d9-node.service
```

2. Add the following content:
```ini
[Unit]
Description=D9 Node
After=network.target

[Service]
Type=simple
User=ubuntu
ExecStart=/usr/local/bin/d9-node \
  --base-path /home/ubuntu/node-data \
  --chain /usr/local/bin/new-main-spec.json \
  --name MY_NODE_NAME \
  --validator

Restart=on-failure

[Install]
WantedBy=multi-user.target
```

3. Enable and start the service:
```bash
sudo systemctl daemon-reload
sudo systemctl enable d9-node
sudo systemctl start d9-node
```

## Docker Installation

For containerized deployments:

```bash
docker pull d9chain/d9-node:latest
docker run -d \
  --name d9-node \
  -p 30333:30333 \
  -p 9944:9944 \
  -v $HOME/node-data:/data \
  d9chain/d9-node:latest \
  --base-path /data \
  --name "MyDockerNode"
```

## Verification

After installation, verify your node is running:

```bash
# Check service status
sudo systemctl status d9-node

# View logs
journalctl -u d9-node.service -f

# Check node info via RPC
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
  http://localhost:9944
```

## Configuration

### Node Options

Common node configuration options:
- `--validator`: Run as a validator
- `--name`: Set your node name
- `--pruning`: Database pruning mode
- `--wasm-execution`: WASM execution method

### Network Configuration

Ensure these ports are open:
- **30333/tcp**: P2P communication
- **9944/tcp**: RPC endpoint (optional)
- **9933/tcp**: WebSocket endpoint (optional)

### Resource Tuning

For optimal performance:
```bash
# Increase file descriptor limit
ulimit -n 65536

# Set in /etc/security/limits.conf
* soft nofile 65536
* hard nofile 65536
```

## Troubleshooting

### Common Issues

1. **Insufficient Disk Space**
   ```bash
   df -h
   # Clean up if needed
   sudo apt autoremove
   ```

2. **Build Failures**
   - Ensure Rust is up to date
   - Check all dependencies are installed
   - Try specific version: `git checkout v1.0.0`

3. **Service Won't Start**
   ```bash
   # Check logs
   journalctl -u d9-node.service -n 50
   # Verify permissions
   ls -la /home/ubuntu/node-data
   ```

4. **Connection Issues**
   - Check firewall settings
   - Verify port forwarding
   - Ensure stable internet

### Getting Help

- [GitHub Issues](https://github.com/D-Nine-Chain/d9_node/issues)
- [Discord Community](https://discord.gg/d9chain)
- [Documentation](https://docs.d9.network)

## Next Steps

- [Running as a Validator](./running-as-a-validator.md)
- [Node Configuration](./node-configuration.md)
- [Monitoring Your Node](./monitoring.md)
- [Security Best Practices](./security.md)