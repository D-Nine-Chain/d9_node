.PHONY: local-alice
local-alice:
	./target/debug/d9-node --base-path /tmp/alice --chain local --alice --port 30333 --ws-port 9944 --rpc-port 9933 --node-key 0000000000000000000000000000000000000000000000000000000000000000 --validator
.PHONY: local-bob
local-bob:
	./target/debug/d9-node --base-path /tmp/bob --chain local --bob --port 30333 --ws-port 9944 --rpc-port 9933 --node-key 0000000000000000000000000000000000000000000000000000000000000001 --validator

.PHONY: dev-alice
dev-alice:
	./target/debug/d9-node --base-path ./dev_chain-state --chain dev --alice --port 30333 --ws-port 9944 --rpc-port 9933 --node-key 0000000000000000000000000000000000000000000000000000000000000000 --validator

.PHONY: dev-bob
dev-bob:
	./target/debug/d9-node --base-path ./dev_chain-state --chain dev --bob --port 30333 --ws-port 9944 --rpc-port 9933 --node-key 0000000000000000000000000000000000000000000000000000000000000001 --validator -