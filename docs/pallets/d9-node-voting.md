# D9 Node Voting Pallet

The D9 Node Voting pallet implements a community-driven validator selection mechanism through voting.

## Overview

The node voting system allows:
- Token holders to vote for validators
- Weighted voting based on stake
- Dynamic validator set updates
- Reward sharing with voters
- Slashing for misbehavior

## Key Concepts

### Voting Mechanism
- One token = one vote
- Votes can be delegated
- Vote weight decreases over time
- Minimum stake requirements

### Validator Selection
- Top N validators by votes become active
- Regular rotation periods
- Grace periods for transitions

### Reward Distribution
- Validators share rewards with voters
- Proportional to voting support
- Compound reward options
- Referral reward sharing

## Extrinsics

### `vote_for_node`
Cast votes for a validator candidate.

```rust
#[pallet::call_index(0)]
pub fn vote_for_node(
    origin: OriginFor<T>,
    candidate: T::AccountId,
    #[pallet::compact] amount: BalanceOf<T>,
) -> DispatchResult
```

### `remove_vote`
Remove votes from a validator.

```rust
#[pallet::call_index(1)]
pub fn remove_vote(
    origin: OriginFor<T>,
    candidate: T::AccountId,
) -> DispatchResult
```

### `register_as_candidate`
Register as a validator candidate.

```rust
#[pallet::call_index(2)]
pub fn register_as_candidate(
    origin: OriginFor<T>,
    commission: Percent,
) -> DispatchResult
```

### `update_commission`
Update validator commission rate.

```rust
#[pallet::call_index(3)]
pub fn update_commission(
    origin: OriginFor<T>,
    new_commission: Percent,
) -> DispatchResult
```

### `claim_rewards`
Claim accumulated voting rewards.

```rust
#[pallet::call_index(4)]
pub fn claim_rewards(origin: OriginFor<T>) -> DispatchResult
```

## Storage

### `CandidateList`
All registered validator candidates.

```rust
StorageValue<_, Vec<T::AccountId>>
```

### `VotingInterests`
Votes cast by each account.

```rust
StorageMap<_, Blake2_128Concat, T::AccountId, VotingInterest<T>>
```

### `CandidateVotes`
Total votes for each candidate.

```rust
StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>>
```

### `ValidatorCommission`
Commission rates for validators.

```rust
StorageMap<_, Blake2_128Concat, T::AccountId, Percent>
```

### `SessionValidators`
Active validators for each session.

```rust
StorageMap<_, Blake2_128Concat, SessionIndex, Vec<T::AccountId>>
```

## Events

- `VoteCast`: Votes cast for validator
- `VoteRemoved`: Votes withdrawn
- `CandidateRegistered`: New validator candidate
- `CommissionUpdated`: Commission rate changed
- `RewardsClaimed`: Voting rewards claimed
- `ValidatorElected`: Validator selected for session

## Errors

- `NotCandidate`: Account not registered as candidate
- `AlreadyVoted`: Already voting for this candidate
- `InsufficientStake`: Below minimum stake requirement
- `NoVotesToRemove`: No votes to withdraw
- `InvalidCommission`: Commission rate out of bounds

## Configuration

```rust
impl pallet_d9_node_voting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type CurrencyBalance = Balance;
    type MinimumStake = MinimumVotingStake;
    type MaxVotesPerVoter = MaxVotesPerVoter;
    type MaxValidators = MaxValidatorNodes;
    type SessionInterface = Session;
    type ReferendumManager = ReferendumManager;
    type RewardManager = D9NodeRewards;
    type WeightInfo = ();
}
```

## Voting Process

### 1. Registration
Validators register with initial commission rate:
```rust
D9NodeVoting::register_as_candidate(origin, Percent::from_percent(10))?;
```

### 2. Voting
Token holders vote for preferred validators:
```rust
D9NodeVoting::vote_for_node(origin, validator, amount)?;
```

### 3. Selection
Top validators by vote weight are selected each era.

### 4. Rewards
Voters claim rewards proportional to their support:
```rust
D9NodeVoting::claim_rewards(origin)?;
```

## Reward Calculation

Rewards are distributed as follows:
1. Validator receives base reward + commission
2. Voters share remaining rewards proportionally
3. Referral bonuses applied if applicable
4. Compound options available

Example calculation:
```
Total Reward: 1000 D9
Validator Commission: 10%
Validator Gets: 100 D9 + share as voter
Voters Share: 900 D9 (proportional to votes)
```

## Integration with Other Pallets

### Node Rewards
Calculates and distributes rewards:
```rust
impl RewardManager<T> for D9NodeRewards {
    fn distribute_rewards(validator: &T::AccountId, amount: Balance) {
        // Distribution logic
    }
}
```

### Council Lock
Coordinates governance decisions:
```rust
impl ReferendumManager for ReferendumManager {
    fn start_pending_votes(session_index: SessionIndex) {
        CouncilLock::start_pending_votes(session_index);
    }
}
```

## Best Practices

1. Research validators before voting
2. Diversify votes across multiple validators
3. Monitor validator performance
4. Claim rewards regularly
5. Adjust votes based on performance

## Security Considerations

- Minimum stake prevents spam
- Vote decay prevents stale votes
- Slashing for misbehavior
- Commission limits

## See Also

- [Node Rewards Pallet](./d9-node-rewards.md)
- [Staking Guide](../staking.md)
- [Validator Guide](../running-as-a-validator.md)