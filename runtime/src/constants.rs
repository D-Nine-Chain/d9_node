//! constants module is set for runtime constants.
//! this is not to be used for pallets constants that is to be defined in parameter_types!

use frame_support::traits::LockIdentifier;
use crate::{ Balance, Block };

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
pub const DAY: BlockNumber = CHAIN_HOURS * 24;
pub const WEEK: BlockNumber = DAY * 7;

//currency
pub const D9_BASE_SUB_UNIT: Balance = 1;
pub const D9_TOKEN: Balance = 1_000_000;
pub const ONE_HUNDRED_D9_TOKENS: Balance = D9_TOKEN * 100;
pub const ONE_THOUSAND_D9_TOKENS: Balance = D9_TOKEN * 1000;
pub const ONE_MILLION_D9_TOKENS: Balance = D9_TOKEN * 1000000;
pub const EXISTENTIAL_DEPOSIT: u128 = 1000;
//election and staking
//candidacy eligibility
pub const CANDIDACY_BOND: Balance = 20 * ONE_THOUSAND_D9_TOKENS;
// base amoun to stake to be permitted to vote
pub const VOTING_BOND_BASE: Balance = 10 * ONE_THOUSAND_D9_TOKENS;
// the cost of a single vote e.g. if 2 then 2 votes cost 4 tokens
pub const VOTING_BOND_FACTOR: Balance = 1;
const STAKING_ID: LockIdentifier = *b"staking";
const ELECTION_LOCK: LockIdentifier = *b"election";
const DESIRED_MEMBERS: u32 = 27;
const DESIRED_RUNNERS_UP: u32 = 100;
const SESSION_PERIOD: BlockNumber = 1 * DAY;
const SESSION_OFFSET: BlockNumber = 1 * MINUTE;
const SESSIONS_PER_ERA: SessionIndex = 1;
const MAX_CANDIDATES: u32 = 200;
const MAX_VOTES_PER_VOTER: u32 = 1;
