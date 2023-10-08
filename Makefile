.PHONY: local-alice
local-alice:
	./target/debug/d9-node --base-path /tmp/alice --chain local --alice --port 39191 --ws-port 39292 --rpc-port 39393 --node-key 0000000000000000000000000000000000000000000000000000000000000000 --validator
.PHONY: local-bob
local-bob:
	./target/debug/d9-node --base-path /tmp/bob --chain local --bob --port 39191 --ws-port 39292 --rpc-port 39393 --node-key 0000000000000000000000000000000000000000000000000000000000000001 --validator

.PHONY: dev-alice
dev-alice:
	./target/debug/d9-node --base-path ./dev_chain_state --chain dev --alice --port 49191  --node-key 0000000000000000000000000000000000000000000000000000000000000000 --validator

.PHONY: dev-bob
dev-bob:
	./target/debug/d9-node --base-path ./dev_chain_state --chain dev --bob --port 49191  --node-key 0000000000000000000000000000000000000000000000000000000000000001 --validator --bootnodes /ip4/

.PHONY: dev-charlie
dev-charlie:
	./target/release/d9-node --base-path ./dev_chain_state --chain ./d9_dev_chain_spec.json --charlie --port 49191 --ws-port 49292 --rpc-port 49393 --rpc-external --node-key 0000000000000000000000000000000000000000000000000000000000000002 --ws-external --rpc-cors=all

# main net ports
#wss: 59292
#rpc: 59393
#p2p 59191