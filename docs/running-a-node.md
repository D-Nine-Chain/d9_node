# Running a D9 Node / 运行 D9 节点

[English](#english) | [中文](#中文)

---

## English

### Managing Your D9 Node

Once your D9 node is installed, you can manage it using systemd commands. Here are the essential commands for daily operations:

#### Check Node Status
```bash
sudo systemctl status d9-node.service
```
This shows if your node is running, stopped, or has encountered any errors.

#### Start the Node
```bash
sudo systemctl start d9-node.service
```
Use this to start your node if it's stopped.

#### Stop the Node
```bash
sudo systemctl stop d9-node.service
```
Use this to safely shut down your node.

#### Restart the Node
```bash
sudo systemctl restart d9-node.service
```
Use this to restart your node (useful after configuration changes).

#### View Real-time Logs
```bash
journalctl -u d9-node.service -f
```
This shows live logs from your node. Press `Ctrl+C` to exit.

#### View Recent Logs
```bash
journalctl -u d9-node.service -n 100
```
Shows the last 100 lines of logs.

#### View Logs from a Specific Time
```bash
journalctl -u d9-node.service --since "1 hour ago"
```
Shows logs from the past hour.

### Common Log Messages

When monitoring your node, you'll see messages like:
- `Peer connected` - Your node connected to another node
- `Imported #12345` - Your node imported a new block
- `Syncing` - Your node is catching up with the network
- `Idle` - Your node is fully synced

### Troubleshooting

If your node won't start:
1. Check the logs: `journalctl -u d9-node.service -n 50`
2. Verify disk space: `df -h`
3. Check system resources: `htop` or `top`
4. Ensure ports are available: `sudo netstat -tlnp | grep 30333`

---

## 中文

### 管理您的 D9 节点

D9 节点安装完成后，您可以使用 systemd 命令进行管理。以下是日常操作的基本命令：

#### 检查节点状态
```bash
sudo systemctl status d9-node.service
```
显示节点是否正在运行、已停止或遇到错误。

#### 启动节点
```bash
sudo systemctl start d9-node.service
```
当节点停止时使用此命令启动。

#### 停止节点
```bash
sudo systemctl stop d9-node.service
```
使用此命令安全关闭节点。

#### 重启节点
```bash
sudo systemctl restart d9-node.service
```
重启节点（在配置更改后很有用）。

#### 查看实时日志
```bash
journalctl -u d9-node.service -f
```
显示节点的实时日志。按 `Ctrl+C` 退出。

#### 查看最近的日志
```bash
journalctl -u d9-node.service -n 100
```
显示最后 100 行日志。

#### 查看特定时间的日志
```bash
journalctl -u d9-node.service --since "1 hour ago"
```
显示过去一小时的日志。

### 常见日志信息

监控节点时，您会看到以下信息：
- `Peer connected` - 您的节点已连接到另一个节点
- `Imported #12345` - 您的节点导入了新区块
- `Syncing` - 您的节点正在与网络同步
- `Idle` - 您的节点已完全同步

### 故障排除

如果节点无法启动：
1. 检查日志：`journalctl -u d9-node.service -n 50`
2. 验证磁盘空间：`df -h`
3. 检查系统资源：`htop` 或 `top`
4. 确保端口可用：`sudo netstat -tlnp | grep 30333`

---

### Quick Reference / 快速参考

| Command / 命令 | Description / 描述 |
|---------------|-------------------|
| `sudo systemctl status d9-node.service` | Check status / 检查状态 |
| `sudo systemctl start d9-node.service` | Start node / 启动节点 |
| `sudo systemctl stop d9-node.service` | Stop node / 停止节点 |
| `sudo systemctl restart d9-node.service` | Restart node / 重启节点 |
| `journalctl -u d9-node.service -f` | Live logs / 实时日志 |
| `journalctl -u d9-node.service -n 100` | Recent logs / 最近日志 |