# D9 Referral Pallet

The D9 Referral pallet manages social relationships and multi-level referral networks within the D9 ecosystem.

## Overview

The referral system creates a social graph that:
- Tracks parent-child relationships between accounts
- Supports up to 19 levels of referral depth
- Enables reward distribution through referral trees
- Provides ancestry queries for any account

## Key Concepts

### Referral Relationships
- **Parent**: The account that referred another account
- **Child**: An account that was referred
- **Ancestors**: All accounts in the referral chain up to the root
- **Direct Referrals**: Immediate children of an account

### Depth Limits
- Maximum referral depth: 19 levels
- Prevents infinite chains
- Ensures efficient traversal


## Storage

### `ReferralRelationships`
Maps each account to its parent account.

```rust
StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId>
```

### `DirectReferralCount`
Number of direct referrals for each account.

```rust
StorageMap<_, Blake2_128Concat, T::AccountId, u32>
```

### `TotalReferralCount`
Total number of referral relationships in the system.

```rust
StorageValue<_, u32>
```

## Query Functions

### `get_parent`
Get the direct parent of an account.

```rust
pub fn get_parent(account: &T::AccountId) -> Option<T::AccountId>
```

### `get_ancestors`
Get all ancestors up to the root.

```rust
pub fn get_ancestors(account: T::AccountId) -> Vec<T::AccountId>
```

### `get_direct_referral_count`
Count of direct children.

```rust
pub fn get_direct_referral_count(account: T::AccountId) -> u32
```

### `validate_referral_depth`
Check if a new referral would exceed depth limit.

```rust
pub fn validate_referral_depth(
    parent: &T::AccountId,
    child: &T::AccountId,
) -> bool
```

## Events

- `ReferralCreated`: New referral relationship established
- `ReferralRemoved`: Referral relationship removed

## Errors

- `ReferralAlreadyExists`: Account already has a parent
- `SelfReferral`: Cannot refer oneself
- `InvalidReferral`: Invalid referral relationship
- `DepthExceeded`: Would exceed maximum depth

## Configuration

```rust
impl pallet_d9_referral::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxReferralDepth = ConstU32<19>;
}
```

## Integration with Other Pallets

### D9 Balances
The balances pallet queries referral relationships during transfers:

```rust
impl ReferralManager<Runtime, ()> for Runtime {
    fn get_parent(account: &AccountId) -> Option<AccountId> {
        D9Referral::get_parent(account)
    }
    
    fn create_referral(parent: &AccountId, child: &AccountId) {
        let _ = D9Referral::create_referral_relationship(parent, child);
    }
}
```

### Node Voting & Rewards
Referral relationships affect reward distribution:
- Voting rewards can be shared with referrers
- Node operators can share profits with their referral network

## Best Practices

1. Always validate referral depth before creating relationships
2. Consider the gas cost of deep ancestor queries
3. Monitor referral statistics for network growth

## Example Usage

```rust
use pallet_d9_referral;

// Create a referral
D9Referral::create_referral(origin, referred_account)?;

// Query ancestry
let ancestors = D9Referral::get_ancestors(account);
for (level, ancestor) in ancestors.iter().enumerate() {
    println!("Level {}: {:?}", level + 1, ancestor);
}

// Check referral count
let direct_referrals = D9Referral::get_direct_referral_count(account);
```

## See Also

- [D9 Balances Pallet](./d9-balances.md)
- [Node Rewards Distribution](./d9-node-rewards.md)
- [Referral Examples](./examples/referral-networks.md)