# D9 Balances Pallet

The D9 Balances pallet extends standard balance management with referral system integration and enhanced token controls.

## Overview

The D9 Balances pallet manages the native D9 token with features including:
- Standard transfer and balance operations
- Integration with the referral system
- Token burns and holds
- Multi-signature support

## Features

### Referral Integration
- Automatically tracks referral relationships during transfers
- Distributes referral rewards through the hierarchy
- Maintains referral statistics

### Enhanced Controls
- Token burns for deflationary mechanics
- Holds for locking tokens temporarily
- Batch transfers for efficiency

## Extrinsics

### `transfer`
Transfer tokens between accounts with referral tracking.

```rust
#[pallet::call_index(0)]
pub fn transfer(
    origin: OriginFor<T>,
    dest: AccountIdLookupOf<T>,
    #[pallet::compact] value: Balance,
) -> DispatchResult
```

### `burn`
Permanently remove tokens from circulation.

```rust
#[pallet::call_index(1)]
pub fn burn(
    origin: OriginFor<T>,
    #[pallet::compact] value: Balance,
) -> DispatchResult
```

### `transfer_with_referral`
Transfer tokens and explicitly set referral relationship.

```rust
#[pallet::call_index(2)]
pub fn transfer_with_referral(
    origin: OriginFor<T>,
    dest: AccountIdLookupOf<T>,
    #[pallet::compact] value: Balance,
    referrer: Option<AccountIdLookupOf<T>>,
) -> DispatchResult
```

## Storage

### `TotalIssuance`
Total number of tokens in circulation.

### `Account`
Account information including free and reserved balance.

### `Locks`
Balance locks for staking, voting, etc.

### `Holds`
Temporary holds on balances.

## Events

- `Transfer`: Tokens transferred between accounts
- `Burn`: Tokens permanently removed
- `ReferralCreated`: New referral relationship established
- `HoldCreated`: Temporary hold placed on balance

## Errors

- `InsufficientBalance`: Account lacks required balance
- `ExistentialDeposit`: Transfer would kill account
- `InvalidReferral`: Referral relationship invalid
- `LiquidityRestrictions`: Balance locked or held

## Configuration

Required runtime configuration:

```rust
impl pallet_d9_balances::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::SubstrateWeight<Runtime>;
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type ReferralManager = Runtime;
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type MaxHolds = MaxHolds;
    type HoldIdentifier = HoldIdentifier;
}
```

## Integration Example

```rust
use pallet_d9_balances;

// Transfer with automatic referral
let result = D9Balances::transfer(
    origin,
    recipient,
    amount,
)?;

// Burn tokens
D9Balances::burn(origin, burn_amount)?;

// Check balance
let balance = D9Balances::free_balance(&account);
```

## Best Practices

1. Always check for sufficient balance before transfers
2. Consider referral implications for all transfers
3. Use holds for temporary locks instead of permanent locks
4. Monitor burn events for tokenomics tracking

## See Also

- [D9 Referral Pallet](./d9-referral.md)
- [Multi-Sig Pallet](./d9-multi-sig.md)
- [Integration Examples](./examples/balance-operations.md)