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
    MSG_ERROR_NOT_64BIT="错误：此脚本只支持 x86_64 架构。ARM64/aarch64 用户请使用源代码构建脚本。"
    MSG_ERROR_DISK_SPACE="错误：您需要至少 60GB 的可用磁盘空间。请清理一些空间后重试。"
    MSG_SWAP_CONFIG="正在配置交换文件..."
    MSG_DOWNLOADING_NODE="正在下载 D9 节点..."
    MSG_DOWNLOADING_HASH="正在下载哈希文件..."
    MSG_VERIFYING_INTEGRITY="正在验证文件完整性..."
    MSG_INTEGRITY_OK="文件完整性验证通过"
    MSG_INTEGRITY_FAILED="文件完整性验证失败"
    MSG_INSTALLING_NODE="正在安装节点..."
    MSG_SETTING_UP_SERVICE="正在设置系统服务..."
    MSG_NODE_NAME_PROMPT="请为您的节点取一个名字："
    MSG_NODE_NAME_INFO="提示：此名字仅对其他节点可见，不会对最终用户显示"
    MSG_CHECKING_KEYS="正在检查现有密钥..."
    MSG_FOUND_KEYS="找到密钥文件："
    MSG_NO_KEYS_FOUND="未找到密钥文件"
    MSG_CREATE_NEW_KEYS="是否创建新密钥？(y/n)"
    MSG_ENTER_SEED="请输入您的种子短语或密钥。如果没有，请按 Enter 创建新的："
    MSG_SEED_TYPE="示例："
    MSG_SEED_MNEMONIC="助记词：word1 word2 word3..."
    MSG_SEED_HEX="密钥：0xb50c46571febcaceeaa161e04dfab28891350759465e7986f77fe790c667607f"
    MSG_GENERATING_KEYS="正在生成新密钥..."
    MSG_IMPORTING_KEYS="正在导入密钥..."
    MSG_NODE_ADDRESS="您的节点地址："
    MSG_INSTALLATION_SUCCESS="安装成功！"
    MSG_NODE_RUNNING="您的节点现在正在后台运行！"
    MSG_CHECK_LOGS="请运行以下命令查看节点状态："
    MSG_DISK_INFO="当前磁盘使用情况："
    MSG_AVAILABLE="可用空间："
    MSG_NEED_SPACE="需要至少 60GB"
    MSG_DOWNLOAD_ERROR="下载失败。请检查网络连接并重试。"
    MSG_VERSION_CHECK="正在检查最新版本..."
    MSG_CHECKING_GLIBC="正在检查 GLIBC 版本..."
    MSG_GLIBC_VERSION="当前 GLIBC 版本："
    MSG_GLIBC_REQUIRED="需要 GLIBC 版本：2.38 或更高"
    MSG_GLIBC_INCOMPATIBLE="GLIBC 版本不兼容"
    MSG_GLIBC_UPGRADING="正在尝试升级 GLIBC..."
    MSG_GLIBC_SUCCESS="GLIBC 成功升级到兼容版本"
    MSG_USE_BUILD_SCRIPT="请使用源代码构建脚本："
else
    # English messages
    MSG_CHECKING_SYSTEM="Checking system requirements..."
    MSG_ERROR_NOT_UBUNTU="Error: This script only supports Ubuntu 22.04. Please make sure you're using Ubuntu 22.04."
    MSG_ERROR_NOT_64BIT="Error: This script only supports x86_64 architecture. ARM64/aarch64 users please use the build from source script."
    MSG_ERROR_DISK_SPACE="Error: You need at least 60GB of free disk space. Please free up some space and try again."
    MSG_SWAP_CONFIG="Configuring swap file..."
    MSG_DOWNLOADING_NODE="Downloading D9 node..."
    MSG_DOWNLOADING_HASH="Downloading hash file..."
    MSG_VERIFYING_INTEGRITY="Verifying file integrity..."
    MSG_INTEGRITY_OK="File integrity verified"
    MSG_INTEGRITY_FAILED="File integrity verification failed"
    MSG_INSTALLING_NODE="Installing node..."
    MSG_SETTING_UP_SERVICE="Setting up system service..."
    MSG_NODE_NAME_PROMPT="Please choose a name for your node:"
    MSG_NODE_NAME_INFO="Note: This name is only visible to other nodes, not to end users"
    MSG_CHECKING_KEYS="Checking for existing keys..."
    MSG_FOUND_KEYS="Found key files:"
    MSG_NO_KEYS_FOUND="No keys found"
    MSG_CREATE_NEW_KEYS="Create new keys? (y/n)"
    MSG_ENTER_SEED="Enter your seed phrase or secret key. Press Enter to create a new one:"
    MSG_SEED_TYPE="Examples:"
    MSG_SEED_MNEMONIC="Mnemonic: word1 word2 word3..."
    MSG_SEED_HEX="Secret: 0xb50c46571febcaceeaa161e04dfab28891350759465e7986f77fe790c667607f"
    MSG_GENERATING_KEYS="Generating new keys..."
    MSG_IMPORTING_KEYS="Importing keys..."
    MSG_NODE_ADDRESS="Your node address:"
    MSG_INSTALLATION_SUCCESS="Installation successful!"
    MSG_NODE_RUNNING="Your node is now running in the background!"
    MSG_CHECK_LOGS="Run this command to check your node status:"
    MSG_DISK_INFO="Current disk usage:"
    MSG_AVAILABLE="Available space:"
    MSG_NEED_SPACE="Need at least 60GB"
    MSG_DOWNLOAD_ERROR="Download failed. Please check your internet connection and try again."
    MSG_VERSION_CHECK="Checking for latest version..."
    MSG_CHECKING_GLIBC="Checking GLIBC version..."
    MSG_GLIBC_VERSION="Current GLIBC version:"
    MSG_GLIBC_REQUIRED="Required GLIBC version: 2.38 or higher"
    MSG_GLIBC_INCOMPATIBLE="GLIBC version is incompatible"
    MSG_GLIBC_UPGRADING="Attempting to upgrade GLIBC..."
    MSG_GLIBC_SUCCESS="GLIBC successfully upgraded to a compatible version"
    MSG_USE_BUILD_SCRIPT="Please use the build from source script:"
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

# Check architecture
ARCH=$(uname -m)
if [ "$ARCH" != "x86_64" ]; then
    explain_error "$MSG_ERROR_NOT_64BIT"
    echo -e "${YELLOW}$MSG_USE_BUILD_SCRIPT${NC}"
    echo -e "curl -sSf https://raw.githubusercontent.com/D-Nine-Chain/d9_node/main/scripts/build-node.sh | bash"
    exit 1
fi

echo -e "${GREEN}✓ Architecture: $ARCH${NC}"

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

# Install dependencies
echo ""
echo -e "${YELLOW}Installing dependencies...${NC}"
sudo apt update -qq
sudo apt install -y -qq curl jq

# Check GLIBC version
echo ""
echo -e "${YELLOW}$MSG_CHECKING_GLIBC${NC}"
# Extract the last version number from the output (the one after the parentheses)
GLIBC_VERSION=$(ldd --version | head -n1 | grep -oE '[0-9]+\.[0-9]+$')
GLIBC_MAJOR=$(echo $GLIBC_VERSION | cut -d. -f1)
GLIBC_MINOR=$(echo $GLIBC_VERSION | cut -d. -f2)
echo -e "${BLUE}$MSG_GLIBC_VERSION $GLIBC_VERSION${NC}"
echo -e "${BLUE}$MSG_GLIBC_REQUIRED${NC}"

# Make sure these are numbers before comparing
if ! [[ "$GLIBC_MAJOR" =~ ^[0-9]+$ ]] || ! [[ "$GLIBC_MINOR" =~ ^[0-9]+$ ]]; then
    echo -e "${RED}Error: Could not parse GLIBC version correctly.${NC}"
    echo -e "${RED}Detected version string: $GLIBC_VERSION${NC}"
    echo -e "${RED}Major: $GLIBC_MAJOR, Minor: $GLIBC_MINOR${NC}"
    exit 1
fi

# Now do the comparison with validated numbers
if [ "$GLIBC_MAJOR" -gt 2 ] || ([ "$GLIBC_MAJOR" -eq 2 ] && [ "$GLIBC_MINOR" -ge 38 ]); then
    echo -e "${GREEN}✓ GLIBC is compatible${NC}"
else
    echo -e "${YELLOW}$MSG_GLIBC_UPGRADING${NC}"
    
    # Backup original sources.list
    sudo cp /etc/apt/sources.list /etc/apt/sources.list.backup
    
    # Add Ubuntu 24.04 repository
    echo -e "${YELLOW}Adding Ubuntu 24.04 repository for newer glibc...${NC}"
    sudo sh -c 'echo "deb http://archive.ubuntu.com/ubuntu noble main" > /etc/apt/sources.list.d/noble.list'
    
    # Update package list
    echo -e "${YELLOW}Updating package lists...${NC}"
    sudo apt update -qq
    
    # Install newer glibc
    echo -e "${YELLOW}Installing newer glibc...${NC}"
    sudo apt install -y -qq libc6
    
    # Verify upgraded version
    GLIBC_VERSION_NEW=$(ldd --version | head -n1 | grep -oE '[0-9]+\.[0-9]+$')
    GLIBC_MAJOR_NEW=$(echo $GLIBC_VERSION_NEW | cut -d. -f1)
    GLIBC_MINOR_NEW=$(echo $GLIBC_VERSION_NEW | cut -d. -f2)
    
    echo -e "${BLUE}New GLIBC version: $GLIBC_VERSION_NEW${NC}"
    
    # Make sure these are numbers before comparing
    if ! [[ "$GLIBC_MAJOR_NEW" =~ ^[0-9]+$ ]] || ! [[ "$GLIBC_MINOR_NEW" =~ ^[0-9]+$ ]]; then
        echo -e "${RED}Error: Could not parse new GLIBC version correctly.${NC}"
        echo -e "${RED}Detected version string: $GLIBC_VERSION_NEW${NC}"
        echo -e "${RED}Major: $GLIBC_MAJOR_NEW, Minor: $GLIBC_MINOR_NEW${NC}"
        exit 1
    fi
    
    if [ "$GLIBC_MAJOR_NEW" -gt 2 ] || ([ "$GLIBC_MAJOR_NEW" -eq 2 ] && [ "$GLIBC_MINOR_NEW" -ge 38 ]); then
        echo -e "${GREEN}✓ $MSG_GLIBC_SUCCESS${NC}"
    else
        echo -e "${RED}✗ $MSG_GLIBC_INCOMPATIBLE${NC}"
        echo -e "${YELLOW}$MSG_USE_BUILD_SCRIPT${NC}"
        echo -e "curl -sSf https://raw.githubusercontent.com/D-Nine-Chain/d9_node/main/scripts/build-node.sh | bash"
        
        # Restore original sources.list
        sudo rm /etc/apt/sources.list.d/noble.list
        sudo apt update -qq
        
        exit 1
    fi
fi

# Get latest release URL
echo ""
echo -e "${YELLOW}$MSG_VERSION_CHECK${NC}"
# Get the latest release JSON and store it
LATEST_RELEASE=$(curl -s https://api.github.com/repos/D-Nine-Chain/d9_node/releases/latest)

# Extract URLs directly from the stored JSON
DOWNLOAD_URL=$(echo "$LATEST_RELEASE" | jq -r '.assets[] | select(.name | endswith(".tar.gz")) | .browser_download_url')
HASH_URL=$(echo "$LATEST_RELEASE" | jq -r '.assets[] | select(.name | endswith(".sha256")) | .browser_download_url')

echo "Download URL: $DOWNLOAD_URL"
echo "Hash URL: $HASH_URL"

if [ -z "$DOWNLOAD_URL" ] || [ -z "$HASH_URL" ]; then
    echo -e "${RED}$MSG_DOWNLOAD_ERROR${NC}"
    echo "Could not find download URLs"
    exit 1
fi

# Download node
echo -e "${YELLOW}$MSG_DOWNLOADING_NODE${NC}"
echo "URL: $DOWNLOAD_URL"
cd $HOME
wget -O d9-node.tar.gz "$DOWNLOAD_URL" || {
    explain_error "$MSG_DOWNLOAD_ERROR"
    exit 1
}

# Download hash file for verification
echo -e "${YELLOW}$MSG_DOWNLOADING_HASH${NC}"
echo "Hash URL: $HASH_URL"
wget -O d9-node.tar.gz.sha256 "$HASH_URL" || {
    explain_error "$MSG_DOWNLOAD_ERROR"
    rm -f d9-node.tar.gz
    exit 1
}

# Verify integrity
echo -e "${YELLOW}$MSG_VERIFYING_INTEGRITY${NC}"
EXPECTED_HASH=$(cat d9-node.tar.gz.sha256 | awk '{print $1}')
ACTUAL_HASH=$(sha256sum d9-node.tar.gz | awk '{print $1}')

if [ "$EXPECTED_HASH" = "$ACTUAL_HASH" ]; then
    echo -e "${GREEN}✓ $MSG_INTEGRITY_OK${NC}"
else
    echo -e "${RED}✗ $MSG_INTEGRITY_FAILED${NC}"
    rm -f d9-node.tar.gz d9-node.tar.gz.sha256
    exit 1
fi

# Extract and install
echo -e "${YELLOW}$MSG_INSTALLING_NODE${NC}"
tar -xzf d9-node.tar.gz
sudo mv d9-node /usr/local/bin/
sudo chmod +x /usr/local/bin/d9-node
rm d9-node.tar.gz d9-node.tar.gz.sha256

# Create data directory
sudo mkdir -p /home/ubuntu/node-data
sudo chown -R $USER:$USER /home/ubuntu/node-data

# Download chain spec
wget -O /tmp/new-main-spec.json https://raw.githubusercontent.com/D-Nine-Chain/d9_node/main/new-main-spec.json
sudo mv /tmp/new-main-spec.json /usr/local/bin/

# Set up systemd service
echo ""
echo -e "${YELLOW}$MSG_SETTING_UP_SERVICE${NC}"

# Ask for node name
echo ""
echo -e "${BLUE}$MSG_NODE_NAME_INFO${NC}"
echo -e "${YELLOW}$MSG_NODE_NAME_PROMPT${NC}"
read -p "> " NODE_NAME

# Create the service file with the user's node name
cat << EOF | sudo tee /etc/systemd/system/d9-node.service > /dev/null
[Unit]
Description=D9 Node
After=network.target

[Service]
Type=simple
User=ubuntu
ExecStart=/usr/local/bin/d9-node \\
  --base-path /home/ubuntu/node-data \\
  --chain /usr/local/bin/new-main-spec.json \\
  --name "$NODE_NAME" \\
  --port 40100 \\
  --validator

Restart=on-failure

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable d9-node.service >/dev/null 2>&1
echo -e "${GREEN}✓ Service configured${NC}"

# Start the service
sudo systemctl start d9-node.service

# Now handle keys
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
    read -p "$MSG_CREATE_NEW_KEYS " create_keys
    
    if [ "$create_keys" = "y" ] || [ "$create_keys" = "Y" ]; then
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
            seed_phrase=$(/usr/local/bin/d9-node key generate --scheme Sr25519 --words 12 | grep "Secret phrase:" | cut -d':' -f2- | xargs)
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
        /usr/local/bin/d9-node key insert \
            --base-path /home/ubuntu/node-data \
            --chain /usr/local/bin/new-main-spec.json \
            --scheme Sr25519 \
            --suri "${seed_phrase}//aura" \
            --key-type aura
        
        /usr/local/bin/d9-node key insert \
            --base-path /home/ubuntu/node-data \
            --chain /usr/local/bin/new-main-spec.json \
            --scheme Ed25519 \
            --suri "${seed_phrase}//grandpa" \
            --key-type gran
        
        /usr/local/bin/d9-node key insert \
            --base-path /home/ubuntu/node-data \
            --chain /usr/local/bin/new-main-spec.json \
            --scheme Sr25519 \
            --suri "${seed_phrase}//im_online" \
            --key-type imon
        
        # Restart the service
        sudo systemctl start d9-node.service
        
        echo -e "${GREEN}✓ Keys imported${NC}"
        
        # Get and display the node address
        ADDRESS_JSON=$(/usr/local/bin/d9-node key inspect --network reynolds --output-type json "${seed_phrase}")
        SS58_ADDRESS=$(echo "$ADDRESS_JSON" | jq -r '.ss58Address')
        
        echo ""
        echo -e "${GREEN}$MSG_NODE_ADDRESS${NC}"
        echo -e "${YELLOW}Dn${SS58_ADDRESS}${NC}"
        echo ""
    fi
else
    echo -e "${GREEN}$MSG_FOUND_KEYS${NC}"
    echo "Aura: $AURA_KEYS, Grandpa: $GRANDPA_KEYS, ImOnline: $IMONLINE_KEYS"
fi

# Success message
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}$MSG_INSTALLATION_SUCCESS${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BLUE}$MSG_NODE_RUNNING${NC}"
echo ""
echo -e "${YELLOW}$MSG_CHECK_LOGS${NC}"
echo -e "${GREEN}journalctl -u d9-node.service -f${NC}"
echo ""
echo "To stop the node:  sudo systemctl stop d9-node.service"
echo "To start the node: sudo systemctl start d9-node.service"
echo "To restart:        sudo systemctl restart d9-node.service"