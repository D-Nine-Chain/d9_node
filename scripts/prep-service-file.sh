#!/usr/bin/env bash

#file assumes the path of the d9-node binary

set -e
NODE_TYPE=$1
ADDITIONAL_FLAGS=$2
IS_VALIDATOR=0 # Initializing the variable

# Ensure the script is run as root
if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root"
   exit 1
fi

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
# Determine configuration file based on the node type
if [[ "$NODE_TYPE" == "validator" ]]; then
   CONFIG_FILE="$SCRIPT_DIR/config_files/d9-validator.conf"
   IS_VALIDATOR=1
elif [[ -z "$NODE_TYPE" ]]; then
   CONFIG_FILE="$SCRIPT_DIR/config_files/d9-node.conf"
else
   echo "Usage: $0 [ validator - for a validator empty otherwise ] (additional_flags)"
   exit 1

fi

# Check if the configuration file exists
if [[ ! -f "$CONFIG_FILE" ]]; then
   echo "Error: Configuration file $CONFIG_FILE not found."
   exit 1
fi

# Load environment variables from the configuration file
source "$CONFIG_FILE"

# Choose whether to append the --validator flag or not
if [ "$IS_VALIDATOR" -eq 1 ]; then
   VALIDATOR_FLAG="--validator"
else
   VALIDATOR_FLAG=""
fi

if [ "$IS_VALIDATOR" -eq 0 ]; then
   ACCEPT_EXTERNAL_FLAG="--rpc-port $RPC_PORT --ws-port $WS_PORT   $ACCEPT_EXTERNAL_FLAG --rpc-external --ws-external --ws-max-connections $WS_MAX_CONNECTIONS --rpc-cors all"
else
   ACCEPT_EXTERNAL_FLAG=""
fi

# Construct the service file
cat <<EOF >./d9-node.service
[Unit]
Description=D9 Node
After=network-online.target

[Service]
User=$SUDO_USER  
ExecStart=$D9_NODE_PATH --base-path $CHAIN_DATA_PATH --chain $CHAIN_SPEC_PATH --port $P2P_PORT $VALIDATOR_FLAG$ACCEPT_EXTERNAL_FLAG --name $NODE_NAME
Restart=on-failure
RestartSec=3
LimitNOFILE=4096

[Install]
WantedBy=multi-user.target
EOF

echo "d9-node service file has been created"
