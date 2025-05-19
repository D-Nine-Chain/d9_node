# D9 Treasury Pallet

The D9 Treasury pallet manages community funds through a proposal-based spending system with council oversight.

## Overview

The treasury system enables:
- Community-controlled fund management
- Proposal-based spending requests
- Council approval mechanism
- Spending limits and controls
- Transparent fund allocation

## Key Features

### Proposal System
- Anyone can submit spending proposals
- Proposals require detailed justification
- Small proposal fee to prevent spam
- Time-limited voting periods

### Council Control
- Only council members can approve spending
- Multi-stage approval process
- Structured voting mechanisms

### Spending Limits
- Maximum per-transaction limits
- Daily/weekly spending caps
- Reserve requirements
- Audit trails

## Extrinsics

### `propose_spend`
Submit a new spending proposal.

```rust
#[pallet::call_index(0)]
pub fn propose_spend(
    origin: OriginFor<T>,
    #[pallet::compact] value: BalanceOf<T>,
    beneficiary: AccountIdLookupOf<T>,
    description: Vec<u8>,
) -> DispatchResult
```

### `approve_proposal`
Council approves a spending proposal.

```rust
#[pallet::call_index(1)]
pub fn approve_proposal(
    origin: OriginFor<T>,
    #[pallet::compact] proposal_id: ProposalIndex,
) -> DispatchResult
```

### `reject_proposal`
Council rejects a spending proposal.

```rust
#[pallet::call_index(2)]
pub fn reject_proposal(
    origin: OriginFor<T>,
    #[pallet::compact] proposal_id: ProposalIndex,
) -> DispatchResult
```

### `spend`
Execute an approved proposal.

```rust
#[pallet::call_index(3)]
pub fn spend(
    origin: OriginFor<T>,
    #[pallet::compact] proposal_id: ProposalIndex,
) -> DispatchResult
```

## Storage

### `Proposals`
All spending proposals and their status.

```rust
StorageMap<_, Blake2_128Concat, ProposalIndex, Proposal<T>>
```

### `ProposalCount`
Total number of proposals submitted.

### `Approvals`
List of approved but unspent proposals.

### `TreasuryBalance`
Current balance in the treasury.

## Events

- `Proposed`: New spending proposal submitted
- `Approved`: Proposal approved by council
- `Rejected`: Proposal rejected
- `Spent`: Funds disbursed from treasury
- `Deposit`: Funds added to treasury

## Errors

- `InsufficientFunds`: Treasury lacks required balance
- `ProposalNotFound`: Invalid proposal ID
- `AlreadyApproved`: Proposal already approved
- `NotCouncilMember`: Caller not authorized
- `ExceedsMaxSpend`: Amount exceeds limits

## Configuration

```rust
impl pallet_d9_treasury::Config for Runtime {
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ProposalFee = ProposalFee;
    type SpendFunds = ();
    type MaxSpendPerTransaction = MaxSpendPerTransaction;
    type ProposalBond = ProposalBond;
    type ProposalBondMinimum = ProposalBondMinimum;
    type ProposalBondMaximum = ProposalBondMaximum;
    type SpendPeriod = SpendPeriod;
    type ApproveOrigin = EnsureTreasurer<Runtime, ()>;
    type RejectOrigin = EnsureTreasurer<Runtime, ()>;
    type SpendOrigin = EnsureTreasurerLimit<Runtime, ()>;
    type WeightInfo = ();
}
```

## Treasury Management

### Funding Sources
1. Transaction fees (percentage)
2. Slashed validator funds
3. Direct contributions
4. Protocol revenue

### Spending Categories
1. Development grants
2. Infrastructure costs
3. Marketing initiatives
4. Community rewards
5. Emergency funds

## Governance Integration

The treasury works with other governance components:

### Council Lock
- Council members vote on proposals
- Voting power based on stake
- Time-locked decisions

### Multi-Sig
- Large spends require multi-sig
- Enhanced security for significant transactions
- Audit requirements

## Example Usage

```rust
use pallet_d9_treasury;

// Submit a proposal
D9Treasury::propose_spend(
    origin,
    amount,
    beneficiary,
    b"Infrastructure upgrade for validators".to_vec(),
)?;

// Council approves
D9Treasury::approve_proposal(council_origin, proposal_id)?;

// Execute spending
D9Treasury::spend(treasury_origin, proposal_id)?;

// Check treasury balance
let balance = D9Treasury::treasury_balance();
```

## Best Practices

1. Provide detailed proposal descriptions
2. Set realistic funding amounts
3. Include milestones for large projects
4. Regular treasury audits
5. Transparent reporting

## Security Considerations

- Multi-signature for large amounts
- Time delays for significant changes
- Regular security audits
- Spending limits and controls

## See Also

- [Council Lock Pallet](./d9-council-lock.md)
- [Multi-Sig Pallet](./d9-multi-sig.md)
- [Treasury Proposals Guide](../guides/treasury-proposals.md)