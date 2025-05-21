# D9节点安装指南

本指南涵盖了D9节点的多种安装方法，从快速自动安装到源代码手动编译。

## 系统要求

- **操作系统**: Ubuntu 22.04 LTS (推荐)
- **架构**: x86_64或ARM64
- **内存**: 最低8GB (推荐16GB)
- **存储**: 最低60GB可用空间 (推荐SSD)
- **网络**: 稳定的互联网连接
- **端口**: 30333 (P2P), 9944 (RPC)

## 快速安装 (推荐)

启动D9节点最快的方法是使用我们的自动安装脚本:

```bash
curl -sSf https://raw.githubusercontent.com/D-Nine-Chain/d9_node/main/scripts/install-d9-node.sh | bash
```

此脚本将会:
1. 验证系统要求
2. 下载最新的预编译二进制文件
3. 配置systemd服务
4. 设置验证者密钥 (可选)
5. 自动启动节点

### 脚本选项

安装脚本支持多种语言:
- 英语 (默认)
- 中文

脚本启动时会提示您选择语言。

### 脚本功能详解

1. **系统检查**
   - 验证Ubuntu 22.04
   - 检查架构 (x86_64/ARM64)
   - 确保有60GB+可用磁盘空间
   - 配置交换文件

2. **节点安装**
   - 从GitHub下载最新版本
   - 安装到 `/usr/local/bin/d9-node`
   - 创建数据目录 `/home/ubuntu/node-data`
   - 下载链规范文件

3. **服务配置**
   - 创建systemd服务文件
   - 启用自动启动
   - 配置日志记录

4. **密钥管理** (可选)
   - 生成或导入验证者密钥
   - 设置密钥存储
   - 显示节点地址

## 从源代码构建

对于开发者或希望从源代码编译的高级用户:

### 使用构建脚本

```bash
curl -sSf https://raw.githubusercontent.com/D-Nine-Chain/d9_node/main/scripts/install-d9-node.sh -o install-d9-node.sh && chmod +x install-d9-node.sh && ./install-d9-node.sh
```

此脚本:
1. 安装所有依赖
2. 设置Rust工具链
3. 克隆代码库
4. 构建节点
5. 配置服务

### 手动构建

#### 1. 安装Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
```

#### 2. 安装系统依赖

```bash
sudo apt update
sudo apt install -y build-essential git clang curl libssl-dev llvm \
    libudev-dev make protobuf-compiler pkg-config
```

#### 3. 克隆并构建

```bash
git clone https://github.com/D-Nine-Chain/d9_node.git
cd d9_node
cargo build --release
```

从源代码构建通常需要15-30分钟，取决于您的系统性能。

#### 4. 安装二进制文件

```bash
sudo cp target/release/d9-node /usr/local/bin/
sudo chmod +x /usr/local/bin/d9-node
```

## 手动服务设置

如果您是手动从源代码构建的，请创建systemd服务:

1. 创建服务文件:
```bash
sudo nano /etc/systemd/system/d9-node.service
```

2. 添加以下内容:
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

3. 启用并启动服务:
```bash
sudo systemctl daemon-reload
sudo systemctl enable d9-node
sudo systemctl start d9-node
```

## Docker安装

对于容器化部署:

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

## 验证安装

安装后，验证您的节点是否运行:

```bash
# 检查服务状态
sudo systemctl status d9-node

# 查看日志
journalctl -u d9-node.service -f

# 通过RPC检查节点信息
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
  http://localhost:9944
```

## 配置

### 节点选项

常用节点配置选项:
- `--validator`: 作为验证者运行
- `--name`: 设置您的节点名称
- `--pruning`: 数据库修剪模式
- `--wasm-execution`: WASM执行方法

### 网络配置

确保这些端口开放:
- **30333/tcp**: P2P通信
- **9944/tcp**: RPC端点 (可选)
- **9933/tcp**: WebSocket端点 (可选)

### 资源调优

为获得最佳性能:
```bash
# 增加文件描述符限制
ulimit -n 65536

# 在/etc/security/limits.conf中设置
* soft nofile 65536
* hard nofile 65536
```

## 故障排除

### 常见问题

1. **磁盘空间不足**
   ```bash
   df -h
   # 如需清理
   sudo apt autoremove
   ```

2. **构建失败**
   - 确保Rust是最新的
   - 检查所有依赖是否已安装
   - 尝试特定版本: `git checkout v1.0.0`

3. **服务无法启动**
   ```bash
   # 检查日志
   journalctl -u d9-node.service -n 50
   # 验证权限
   ls -la /home/ubuntu/node-data
   ```

4. **连接问题**
   - 检查防火墙设置
   - 验证端口转发
   - 确保互联网稳定

### 获取帮助

- [GitHub问题](https://github.com/D-Nine-Chain/d9_node/issues)
- [Discord社区](https://discord.gg/d9chain)
- [文档](https://docs.d9.network)

## 后续步骤

- [作为验证者运行](./running-as-a-validator.md)
- [节点配置](./node-configuration.md)
- [监控您的节点](./monitoring.md)
- [安全最佳实践](./security.md)