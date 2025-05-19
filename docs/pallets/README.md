# D9 Chain Custom Pallets

D9 Chain implements several custom pallets that provide unique functionality beyond standard Substrate pallets. These pallets work together to create a comprehensive governance and economic system.

## Overview

| Pallet | Purpose | Key Features |
|--------|---------|--------------|
| [D9 Balances](./d9-balances.md) | Token management | Referral integration, burns, holds |
| [D9 Referral](./d9-referral.md) | Social graph | Multi-level referrals, ancestry tracking |
| [D9 Treasury](./d9-treasury.md) | Fund management | Proposals, spending limits, council control |
| [D9 Node Voting](./d9-node-voting.md) | Validator selection | Community voting, reward distribution |
| [D9 Node Rewards](./d9-node-rewards.md) | Incentive system | Automatic reward calculation and distribution |
| [D9 Council Lock](./d9-council-lock.md) | Governance | Council membership, voting rounds |
| [D9 Multi Sig](./d9-multi-sig.md) | Shared control | Multi-signature operations, time locks |

## Pallet Interactions

The D9 pallets are designed to work together:

1. **Referral → Balances**: Referral relationships affect token transfers and rewards
2. **Node Voting → Node Rewards**: Voting determines validator rewards distribution
3. **Treasury → Council Lock**: Council members control treasury spending
4. **Multi Sig → All Pallets**: Multi-signature can control any pallet operation

## Key Concepts

### Referral System
- Tracks parent-child relationships between accounts
- Supports up to 19 levels of referral depth
- Distributes rewards through the referral tree

### Validator Selection
- Community votes for validators
- Rewards distributed based on voting support
- Automatic reward calculation each era

### Treasury Management
- Proposal-based spending
- Council approval required
- Maximum spend limits per transaction

### Governance
- Council-based decision making
- Time-locked voting rounds
- Proposal fees to prevent spam

## Technical Details

All D9 pallets follow Substrate best practices:
- Proper weight accounting
- Comprehensive error handling
- Event emission for all state changes
- Extensive unit and integration tests
- Benchmarking for accurate weight calculation

## Integration Guide

To integrate with D9 pallets in your runtime:

```rust
use pallet_d9_referral;
use pallet_d9_balances;
use pallet_d9_treasury;
// ... other imports

impl pallet_d9_referral::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxReferralDepth = MaxReferralDepth;
}

impl pallet_d9_balances::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ReferralManager = Runtime;
    // ... other configuration
}
```

## Next Steps

- Read individual pallet documentation for detailed information
- See [integration examples](./examples) for common use cases
- Check [runtime configuration](../runtime-configuration.md) for setup details