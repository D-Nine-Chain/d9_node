#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Language selection
echo "======================="
echo "D9 Node Installation"
echo "======================="
echo ""
echo "Choose your language / 选择您的语言:"
echo "1) English"
echo "2) 中文"
echo ""
read -p "Enter your choice (1 or 2): " lang_choice

# Set messages based on language
if [ "$lang_choice" = "2" ]; then
    # Chinese messages
    MSG_CHECKING_SYSTEM="正在检查系统要求..."
    MSG_ERROR_NOT_UBUNTU="错误：此脚本仅支持 Ubuntu 22.04。请确保您使用的是 Ubuntu 22.04 系统。"
    MSG_ERROR_NOT_I386="错误：此脚本不支持 i386 架构。请使用 64 位系统。"
    MSG_ERROR_DISK_SPACE="错误：您需要至少 60GB 的可用磁盘空间。请清理一些空间后重试。"
    MSG_SWAP_CONFIG="正在配置交换文件..."
    MSG_UPDATING_SYSTEM="正在更新系统软件包..."
    MSG_INSTALLING_DEPS="正在安装必要的依赖项..."
    MSG_INSTALLING_RUST="正在安装 Rust 编程语言..."
    MSG_CLONING_REPO="正在下载 D9 节点代码..."
    MSG_BUILDING_NODE="正在构建节点（这可能需要 10-30 分钟）..."
    MSG_SETTING_UP_SERVICE="正在设置系统服务..."
    MSG_CHECKING_KEYS="正在检查现有密钥..."
    MSG_FOUND_KEYS="找到密钥文件："
    MSG_NO_KEYS_FOUND="未找到密钥文件"
    MSG_CREATE_NEW_KEYS="节点构建成功！现在需要创建密钥。"
    MSG_ENTER_SEED="请输入您的助记词或密钥种子。如果没有，请按 Enter 创建新的："
    MSG_SEED_TYPE="示例："
    MSG_SEED_MNEMONIC="助记词：word1 word2 word3..."
    MSG_SEED_HEX="密钥种子：0xb50c46571febcaceeaa161e04dfab28891350759465e7986f77fe790c667607f"
    MSG_GENERATING_KEYS="正在生成新密钥..."
    MSG_IMPORTING_KEYS="正在导入密钥..."
    MSG_NODE_ADDRESS="您的节点地址："
    MSG_BUILD_SUCCESS="安装成功！"
    MSG_CHECKING_NODE="正在检查节点连接..."
    MSG_CHECK_LOGS="请运行以下命令查看节点状态："
    MSG_LOOK_FOR="您应该看到类似这样的信息："
    MSG_PEERS_INFO="• 已连接到其他节点（peer）"
    MSG_BLOCKS_INFO="• 正在同步区块"
    MSG_PRESS_CTRL_C="按 Ctrl+C 退出日志查看"
    MSG_NODE_RUNNING="您的节点现在正在后台运行！"
    MSG_DISK_INFO="当前磁盘使用情况："
    MSG_AVAILABLE="可用空间："
    MSG_NEED_SPACE="需要至少 60GB"
    MSG_RUST_INSTALLED="Rust 已安装，版本："
    MSG_UPDATING_RUST="正在更新 Rust 到版本 1.75.0..."
    MSG_EXISTING_BUILD="发现已存在的节点构建，跳过构建步骤..."
else
    # English messages
    MSG_CHECKING_SYSTEM="Checking system requirements..."
    MSG_ERROR_NOT_UBUNTU="Error: This script only supports Ubuntu 22.04. Please make sure you're using Ubuntu 22.04."
    MSG_ERROR_NOT_I386="Error: This script does not support i386 architecture. Please use a 64-bit system."
    MSG_ERROR_DISK_SPACE="Error: You need at least 60GB of free disk space. Please free up some space and try again."
    MSG_SWAP_CONFIG="Configuring swap file..."
    MSG_UPDATING_SYSTEM="Updating system packages..."
    MSG_INSTALLING_DEPS="Installing required dependencies..."
    MSG_INSTALLING_RUST="Installing Rust programming language..."
    MSG_CLONING_REPO="Downloading D9 node code..."
    MSG_BUILDING_NODE="Building the node (this may take 10-30 minutes)..."
    MSG_SETTING_UP_SERVICE="Setting up system service..."
    MSG_CHECKING_KEYS="Checking for existing keys..."
    MSG_FOUND_KEYS="Found key files:"
    MSG_NO_KEYS_FOUND="No keys found"
    MSG_CREATE_NEW_KEYS="Node built successfully! Now we need to create keys."
    MSG_ENTER_SEED="Enter your mnemonic phrase or secret seed. Press Enter to create a new one:"
    MSG_SEED_TYPE="Examples:"
    MSG_SEED_MNEMONIC="Mnemonic: word1 word2 word3..."
    MSG_SEED_HEX="Secret seed: 0xb50c46571febcaceeaa161e04dfab28891350759465e7986f77fe790c667607f"
    MSG_GENERATING_KEYS="Generating new keys..."
    MSG_IMPORTING_KEYS="Importing keys..."
    MSG_NODE_ADDRESS="Your node address:"
    MSG_BUILD_SUCCESS="Installation successful!"
    MSG_CHECKING_NODE="Checking node connection..."
    MSG_CHECK_LOGS="Run this command to check your node status:"
    MSG_LOOK_FOR="You should see something like:"
    MSG_PEERS_INFO="• Connected to other nodes (peers)"
    MSG_BLOCKS_INFO="• Syncing blocks"
    MSG_PRESS_CTRL_C="Press Ctrl+C to exit log viewing"
    MSG_NODE_RUNNING="Your node is now running in the background!"
    MSG_DISK_INFO="Current disk usage:"
    MSG_AVAILABLE="Available space:"
    MSG_NEED_SPACE="Need at least 60GB"
    MSG_RUST_INSTALLED="Rust is installed, version:"
    MSG_UPDATING_RUST="Updating Rust to version 1.75.0..."
    MSG_EXISTING_BUILD="Found existing node build, skipping build step..."
fi

# Function to explain errors clearly
explain_error() {
    echo ""
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${RED}$1${NC}"
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

# Check system requirements
echo ""
echo -e "${YELLOW}$MSG_CHECKING_SYSTEM${NC}"
echo ""

# Check Ubuntu 22
if [ -f /etc/os-release ]; then
    . /etc/os-release
    if [ "$ID" != "ubuntu" ] || [ "${VERSION_ID}" != "22.04" ]; then
        explain_error "$MSG_ERROR_NOT_UBUNTU"
        exit 1
    fi
else
    explain_error "$MSG_ERROR_NOT_UBUNTU"
    exit 1
fi

echo -e "${GREEN}✓ Ubuntu 22.04${NC}"

# Check processor architecture (exclude i386)
ARCH=$(uname -m)
if [[ "$ARCH" == "i386" ]]; then
    explain_error "$MSG_ERROR_NOT_I386"
    exit 1
fi

echo -e "${GREEN}✓ Processor architecture: $ARCH${NC}"

# Check disk space
echo ""
echo -e "${BLUE}$MSG_DISK_INFO${NC}"
df -h /
echo ""

AVAILABLE_SPACE=$(df / | awk 'NR==2 {print $4}')
REQUIRED_SPACE=$((60*1024*1024))  # 60GB in KB

echo -e "${BLUE}$MSG_AVAILABLE $(df -h / | awk 'NR==2 {print $4}')${NC}"
echo -e "${BLUE}$MSG_NEED_SPACE${NC}"

if [ "$AVAILABLE_SPACE" -lt "$REQUIRED_SPACE" ]; then
    explain_error "$MSG_ERROR_DISK_SPACE"
    exit 1
fi

echo -e "${GREEN}✓ Sufficient disk space${NC}"

# Configure swap
echo ""
echo -e "${YELLOW}$MSG_SWAP_CONFIG${NC}"
sudo swapoff -a 2>/dev/null || true
if [ -f /swapfile ]; then
    sudo rm /swapfile
fi
sudo fallocate -l 1G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile >/dev/null 2>&1
sudo swapon /swapfile
sudo sh -c 'grep -v "^/swapfile" /etc/fstab > /etc/fstab.tmp && mv /etc/fstab.tmp /etc/fstab'
sudo sh -c 'echo "/swapfile none swap sw 0 0" >> /etc/fstab'
echo -e "${GREEN}✓ Swap configured (1GB)${NC}"

# Update system
echo ""
echo -e "${YELLOW}$MSG_UPDATING_SYSTEM${NC}"
sudo apt update -qq && sudo apt upgrade -y -qq

# Install dependencies (including jq)
echo ""
echo -e "${YELLOW}$MSG_INSTALLING_DEPS${NC}"
sudo apt install -y -qq build-essential git clang curl libssl-dev llvm libudev-dev make protobuf-compiler jq

# Install Rust if not already installed
if ! command -v rustc &> /dev/null; then
    echo ""
    echo -e "${YELLOW}$MSG_INSTALLING_RUST${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
    source "$HOME/.cargo/env"
else
    echo -e "${BLUE}Rust is already installed${NC}"
fi

# Source cargo environment
source "$HOME/.cargo/env"

# Clone D9 node repository
echo ""
echo -e "${YELLOW}$MSG_CLONING_REPO${NC}"
cd $HOME
if [ -d "d9_node" ]; then
    cd d9_node && git pull
else
    git clone https://github.com/D-Nine-Chain/d9_node.git
    cd d9_node
fi

# If rust-toolchain.toml exists, it will override our Rust version
if [ -f "rust-toolchain.toml" ]; then
    echo -e "${BLUE}Using Rust version specified in rust-toolchain.toml${NC}"
fi

# Check if node is already built
if [ -f "./target/release/d9-node" ]; then
    echo -e "${GREEN}$MSG_EXISTING_BUILD${NC}"
else
    # Build the node
    echo ""
    echo -e "${YELLOW}$MSG_BUILDING_NODE${NC}"
    echo -e "${BLUE}Please be patient...${NC}"
    
    if cargo build --release; then
        echo -e "${GREEN}✓ Node built successfully${NC}"
    else
        echo -e "${RED}✗ Build failed. Please check the error messages above.${NC}"
        echo ""
        echo -e "${RED}Common solutions:${NC}"
        echo "1. Try updating to a specific commit:"
        echo "   git checkout <known-good-commit>"
        echo "2. Check for build requirements in README.md"
        echo "3. Report the issue to the D9 team"
        exit 1
    fi
fi

# Set up systemd service
echo ""
echo -e "${YELLOW}$MSG_SETTING_UP_SERVICE${NC}"
sudo cp d9-node.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable d9-node.service >/dev/null 2>&1
echo -e "${GREEN}✓ Service configured${NC}"

# Start the service
sudo systemctl start d9-node.service

# Now handle keys after the node is running
KEYSTORE_PATH="/home/ubuntu/node-data/chains/d9_main/keystore"
echo ""
echo -e "${YELLOW}$MSG_CHECKING_KEYS${NC}"

# Create keystore directory if it doesn't exist
sudo mkdir -p "$KEYSTORE_PATH"
sudo chown -R $USER:$USER /home/ubuntu/node-data

KEY_COUNT=0
if [ -d "$KEYSTORE_PATH" ]; then
    # Count files with required prefixes
    AURA_KEYS=$(ls $KEYSTORE_PATH/617572* 2>/dev/null | wc -l)
    GRANDPA_KEYS=$(ls $KEYSTORE_PATH/6772616e* 2>/dev/null | wc -l)
    IMONLINE_KEYS=$(ls $KEYSTORE_PATH/696d6f6e* 2>/dev/null | wc -l)
    KEY_COUNT=$((AURA_KEYS + GRANDPA_KEYS + IMONLINE_KEYS))
fi

if [ "$KEY_COUNT" -eq 0 ]; then
    echo -e "${YELLOW}$MSG_NO_KEYS_FOUND${NC}"
    echo ""
    echo -e "${BLUE}$MSG_CREATE_NEW_KEYS${NC}"
    echo ""
    
    # Stop the service temporarily to insert keys
    sudo systemctl stop d9-node.service
    
    echo "$MSG_ENTER_SEED"
    echo "$MSG_SEED_TYPE"
    echo "$MSG_SEED_MNEMONIC"
    echo "$MSG_SEED_HEX"
    echo ""
    read -p "> " seed_phrase
    
    if [ -z "$seed_phrase" ]; then
        echo -e "${YELLOW}$MSG_GENERATING_KEYS${NC}"
        # Generate new seed using the node itself
        seed_phrase=$(./target/release/d9-node key generate --scheme Sr25519 --words 12 | grep "Secret phrase:" | cut -d':' -f2- | xargs)
        echo ""
        echo -e "${GREEN}IMPORTANT - SAVE THIS SEED PHRASE:${NC}"
        echo -e "${YELLOW}$seed_phrase${NC}"
        echo ""
        echo "Please write this down and keep it safe!"
        echo "Press Enter when you've saved it..."
        read
    fi
    
    echo -e "${YELLOW}$MSG_IMPORTING_KEYS${NC}"
    
    # Insert the three required keys
    ./target/release/d9-node key insert \
        --base-path /home/ubuntu/node-data \
        --chain new-main-spec.json \
        --scheme Sr25519 \
        --suri "${seed_phrase}//aura" \
        --key-type aura
    
    ./target/release/d9-node key insert \
        --base-path /home/ubuntu/node-data \
        --chain new-main-spec.json \
        --scheme Ed25519 \
        --suri "${seed_phrase}//grandpa" \
        --key-type gran
    
    ./target/release/d9-node key insert \
        --base-path /home/ubuntu/node-data \
        --chain new-main-spec.json \
        --scheme Sr25519 \
        --suri "${seed_phrase}//im_online" \
        --key-type imon
    
    # Restart the service
    sudo systemctl start d9-node.service
    
    echo -e "${GREEN}✓ Keys imported${NC}"
    
    # Get and display the node address using key inspect
    ADDRESS_JSON=$(./target/release/d9-node key inspect --network reynolds --output-type json "${seed_phrase}")
    SS58_ADDRESS=$(echo "$ADDRESS_JSON" | jq -r '.ss58Address')
    
    echo ""
    echo -e "${GREEN}$MSG_NODE_ADDRESS${NC}"
    echo -e "${YELLOW}Dn${SS58_ADDRESS}${NC}"
    echo ""
else
    echo -e "${GREEN}$MSG_FOUND_KEYS${NC}"
    echo "Aura: $AURA_KEYS, Grandpa: $GRANDPA_KEYS, ImOnline: $IMONLINE_KEYS"
fi

# Save the seed phrase for address display if available
if [ -n "$seed_phrase" ]; then
    ADDRESS_JSON=$(./target/release/d9-node key inspect --network reynolds --output-type json "${seed_phrase}")
    SS58_ADDRESS=$(echo "$ADDRESS_JSON" | jq -r '.ss58Address')
    
    echo ""
    echo -e "${GREEN}$MSG_NODE_ADDRESS${NC}"
    echo -e "${YELLOW}Dn${SS58_ADDRESS}${NC}"
    echo ""
fi

# Success message
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}$MSG_BUILD_SUCCESS${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BLUE}$MSG_NODE_RUNNING${NC}"
echo ""
echo -e "${YELLOW}$MSG_CHECK_LOGS${NC}"
echo -e "${GREEN}journalctl -u d9-node.service -f${NC}"
echo ""
echo -e "${BLUE}$MSG_LOOK_FOR${NC}"
echo "$MSG_PEERS_INFO"
echo "$MSG_BLOCKS_INFO"
echo ""
echo -e "${BLUE}$MSG_PRESS_CTRL_C${NC}"