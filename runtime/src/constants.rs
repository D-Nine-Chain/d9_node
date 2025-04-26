//! constants module is set for runtime constants.
//! this is not to be used for pallets constants that is to be defined in parameter_types!

use crate::{Balance, BlockNumber};
use frame_support::traits::LockIdentifier;
use sp_runtime::Perbill;
use sp_staking::{EraIndex, SessionIndex};

//chain
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

//block
// Time as measured in blocks
// NOTE: Currently it is not possible to change the slot duration after the chain has started.
// Attempting to do so will brick block production.
pub const MILLISECS_PER_BLOCK: u64 = 3000;
// pub const BLOCKS_PER_METRIC_MINUTE: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const MINUTE: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOUR: BlockNumber = MINUTE * 60;
pub const DAY: BlockNumber = HOUR * 24;
pub const WEEK: BlockNumber = DAY * 7;

//currency
pub const D9_BASE_SUB_UNIT: Balance = 1;
pub const D9_TOKEN: Balance = 1_000_000_000_000;
pub const ONE_HUNDRED_D9_TOKENS: Balance = D9_TOKEN * 100;
pub const ONE_THOUSAND_D9_TOKENS: Balance = D9_TOKEN * 1000;
pub const ONE_MILLION_D9_TOKENS: Balance = D9_TOKEN * 1000000;
pub const EXISTENTIAL_DEPOSIT: u128 = 1000;

/**
 * Substrate Staking Pallet - Sessions and Eras:
 *
 * - Session:
 *   A fixed number of blocks where the validator set remains unchanged.
 *   Validators are accountable for their actions during each session.
 *
 * - Era:
 *   Comprises multiple sessions.
 *   At the end of an era, staking rewards are distributed and any slashes are applied.
 *   Validator set selection logic typically runs at era end.
 *
 * Relationship:
 * Multiple sessions constitute one era. The exact number is chain-specific and configurable.
 */
// note the sessions per era may have to be increased to improve performance
//candidacy eligibility
/// cost to become a validator candidate
pub const CANDIDACY_BOND: Balance = 20 * ONE_THOUSAND_D9_TOKENS;
/// base amount to stake to be permitted to vote
pub const VOTING_BOND_BASE: Balance = 10 * ONE_THOUSAND_D9_TOKENS;
/// Factor determining the cost of a bond required for voting.
pub const VOTING_BOND_FACTOR: Balance = 1;
/// Unique identifier for locks related to staking.
pub const STAKING_ID: LockIdentifier = *b"staking/";
/// Unique identifier for locks related to elections.
pub const ELECTION_LOCK: LockIdentifier = *b"election";
/// Desired number of members for the validator set.
pub const MAX_VALIDATOR_NODES: u32 = 27;
/// Desired number of backup candidates or runners-up.
pub const DESIRED_RUNNERS_UP: u32 = 100;
/// max N of electable validators
pub const MAX_ON_CHAIN_ELECTABLE_TARGETS: u32 = 200;
/// max N of voters
pub const MAX_ON_CHAIN_ELECTING_VOTERS: u32 = 1000;
/// Number of blocks constituting a session.
pub const SESSION_PERIOD: BlockNumber = 1 * DAY;
/// Offset time before starting the next session.
pub const SESSION_OFFSET: BlockNumber = 1 * MINUTE;
/// wait time in era duration until a slash is executed
pub const SLASH_DEFER_DURATION: EraIndex = 0;
/// Number of sessions within a single era.
pub const SESSIONS_PER_ERA: SessionIndex = 1;
/// Number of eras that staked funds must remain bonded.
pub const BONDING_DURATION: EraIndex = 7;
/// max number of candidates that can be in a single eleection
pub const MAX_CANDIDATES: u32 = 300;
/// max nominations per voter per election
pub const MAX_NOMINATIONS: u32 = 3;
/// max number of votes per voter per election
pub const MAX_VOTES_PER_VOTER: u32 = 1;
/// max nominators than can be rewarded per valdiator
pub const MAX_NOMINATORS_REWARDED_PER_VALIDATOR: u32 = 64;
/// an arbitrary buffer to ensure that the history depth is always greater than the bonding duration
const HISTORY_DEPTH_BUFFER: u32 = 1;
/// N eras' worth of information to retain in history.
/// in this case the 2 is arbitrarily chosen.
pub const HISTORY_DEPTH: u32 = BONDING_DURATION * 2 + HISTORY_DEPTH_BUFFER;
/// The fraction of the validator set that is safe to be offending.
/// After the threshold is reached a new era will be forced.
pub const OFFENDING_VALIDATORS_THRESHOLD: Perbill = Perbill::from_percent(30);
/// Maximum number of `unlocking` chunks a staker can have.
///
/// `MAX_UNLOCKING_CHUNKS` represents the upper limit on the number of unlocking chunks that a
/// [`StakingLedger`] can contain. An unlocking chunk refers to a portion of staked funds that
/// are scheduled to become accessible or "unlocked" at a future era. This value effectively
/// determines the maximum number of unique eras a staker may be unbonding their tokens in.
///
/// The concept of unlocking chunks is central to the process of unbonding, a key feature of
/// the staking system. When a staker initiates the unbonding process, their tokens don't
/// immediately become available. Instead, these tokens are released in chunks over a period
/// of time, promoting network stability by discouraging sudden, large-scale withdrawals.
///
/// In our network, we have chosen `MAX_UNLOCKING_CHUNKS` to be 15. This means a staker can
/// concurrently be in the process of unbonding their tokens in up to 15 different eras. If
/// they attempt to initiate unbonding in a 16th era, they will have to wait until one of the
/// previous unbonding processes is completed.
///
/// It's important to note that reducing `MAX_UNLOCKING_CHUNKS` to a lower value can result in
/// inconsistencies if stakers are already unbonding in more eras than the new maximum.
/// Therefore, any change to `MAX_UNLOCKING_CHUNKS`, particularly a decrease, should be handled
/// via a carefully managed runtime migration.
pub const MAX_UNLOCKING_CHUNKS: u32 = 15;

//contracts
pub const DEPOSIT_PER_ITEM: Balance = 1_000;
pub const DEPOSIT_PER_BYTE: Balance = 1_000;
pub const DEFAULT_DEPOSIT_LIMIT: Balance = 3 * D9_TOKEN;
pub const MAX_CODE_SIZE: u32 = 500 * 1024;
pub const MAX_STORAGE_KEY_LENGTH: u32 = 128;
pub const MAX_DEBUG_BUFFER_LENGTH: u32 = 2 * 1024 * 1024;
