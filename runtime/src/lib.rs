#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
pub mod constants;
pub use crate::constants::*;
use frame_system::EnsureRoot;
use frame_support::traits::{ U128CurrencyToVote, LockIdentifier };
use frame_support::PalletId;
use pallet_babe::ExternalTrigger;
use pallet_grandpa::AuthorityId as GrandpaId;
use pallet_transaction_payment::{ FeeDetails, RuntimeDispatchInfo };
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_api::impl_runtime_apis;
use sp_staking::{ EraIndex, SessionIndex };
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_core::{ crypto::KeyTypeId, OpaqueMetadata };
use sp_inherents::{ CheckInherentsResult, InherentData };
use sp_runtime::{
	create_runtime_str,
	generic,
	impl_opaque_keys,
	traits::{
		AccountIdLookup,
		BlakeTwo256,
		Block as BlockT,
		IdentifyAccount,
		NumberFor,
		One,
		Verify,
	},
	transaction_validity::{ TransactionSource, TransactionValidity, TransactionPriority },
	ApplyExtrinsicResult,
	MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
use frame_election_provider_support::{ onchain, SequentialPhragmen };
// A few exports that help ease life for downstream crates.
pub use frame_support::{
	construct_runtime,
	parameter_types,
	traits::{ ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, StorageInfo },
	weights::{
		constants::{
			BlockExecutionWeight,
			ExtrinsicBaseWeight,
			RocksDbWeight,
			WEIGHT_REF_TIME_PER_SECOND,
		},
		IdentityFee,
		Weight,
	},
	StorageValue,
};
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ ConstFeeMultiplier, CurrencyAdapter, Multiplier };
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{ Perbill, Permill };
/// Import the template pallet.
// pub use pallet_template;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

type Moment = u64;
/// Balance of an account.
pub type Balance = u128;
/// Index of a transaction in the chain.
pub type Index = u32;
/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

pub type Nonce = u32;
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
	sp_consensus_babe::BabeEpochConfiguration {
		c: PRIMARY_PROBABILITY,
		allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
	};
/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
	impl_opaque_keys! {
		pub struct SessionKeys {
			pub babe: Babe,
			pub grandpa: Grandpa,
			pub im_online: ImOnline,
			pub authority_discovery: AuthorityDiscovery,
		}
	}
}

// To learn more about runtime versioning, see:
// https://docs.substrate.io/main-docs/build/upgrade#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("d9"),
	impl_name: create_runtime_str!("d9"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 100,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// The BABE epoch configuration at genesis.
// pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
// 	sp_consensus_babe::BabeEpochConfiguration {
// 		c: PRIMARY_PROBABILITY,
// 		allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
// 	};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const Version: RuntimeVersion = VERSION;
	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::with_sensible_defaults(
			Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
			NORMAL_DISPATCH_RATIO,
		);
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime.
impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = frame_support::traits::Everything;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = BlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = BlockLength;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The set code logic, just the default since we're not a parachain.
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_offences::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
}
parameter_types! {
	pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	/// We prioritize im-online heartbeats over election solution submission.
	pub const StakingUnsignedPriority: TransactionPriority = TransactionPriority::max_value() / 2;
	pub const MaxKeys: u32 = 10_000;
	pub const MaxPeerInHeartbeats: u32 = 10_000;
	pub const MaxPeerDataEncodingSize: u32 = 1_000_000;
}
impl pallet_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type RuntimeEvent = RuntimeEvent;
	type NextSessionRotation = Babe;
	type ValidatorSet = Historical;
	type ReportUnresponsiveness = Offences;
	type UnsignedPriority = ImOnlineUnsignedPriority;
	type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
	type MaxKeys = MaxKeys;
	type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
	type MaxPeerDataEncodingSize = MaxPeerDataEncodingSize;
}

parameter_types! {
	/// changing this value after genesis will brick chain
	pub const EpochDuration: u64 = SESSIONS_PER_ERA as u64;
	pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
   pub ReportLongevity: u64 =
		BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
   pub const MaxAuthorities: u32 = DESIRED_MEMBERS;
}

impl pallet_authority_discovery::Config for Runtime {
	type MaxAuthorities = MaxAuthorities;
}

impl pallet_babe::Config for Runtime {
	type MaxAuthorities = MaxAuthorities;
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ExpectedBlockTime;
	type EpochChangeTrigger = ExternalTrigger;
	type DisabledValidators = Session;
	type KeyOwnerProof = <Historical as KeyOwnerProofSystem<
		(KeyTypeId, pallet_babe::AuthorityId)
	>>::Proof;
	type EquivocationReportSystem = pallet_babe::EquivocationReportSystem<
		Self,
		Offences,
		Historical,
		ReportLongevity
	>;
	type WeightInfo = ();
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	type EventHandler = (Staking, ImOnline);
}

impl pallet_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;

	type WeightInfo = ();
	type MaxAuthorities = MaxAuthorities;
	type MaxSetIdSessionEntries = ConstU64<0>;

	type KeyOwnerProof = sp_core::Void;
	type EquivocationReportSystem = ();
}
parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}
impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = Babe;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime where RuntimeCall: From<C> {
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = RuntimeCall;
}

parameter_types! {
	pub const MaxLocks: u32 = 50;
	pub const ExistentialDeposit: u128 = EXISTENTIAL_DEPOSIT;
	pub const ReserveIdentifier: &'static [u8; 7] = b"reserve";
	pub const MaxHolds: u32 = 50;
	pub const MaxReserves: u32 = 50;
	pub const MaxFreezes: u32 = 50;
	pub const HoldIdentifier: &'static [u8; 4] = b"hold";
}
impl pallet_balances::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type ReserveIdentifier = ();
	type FreezeIdentifier = ();
	type MaxLocks = MaxLocks;
	type MaxHolds = MaxHolds;
	type MaxReserves = MaxReserves;
	type MaxFreezes = MaxFreezes;
	type HoldIdentifier = ();
}

parameter_types! {
	pub FeeMultiplier: Multiplier = Multiplier::one();
   pub const OperationalFeeMultiplier:u8 = 5;
}
impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = IdentityFee<Balance>;
	type LengthToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}

parameter_types! {
	pub const MaxWinners: u32 = DESIRED_MEMBERS;
	pub const TargetsBound: u32 = DESIRED_RUNNERS_UP;
}
// type Solver = frame_election_provider_support::SequentialPhragmen<AccountId, Perbill>;
impl frame_election_provider_support::onchain::Config for Runtime {
	// type System = frame_system::Pallet<Runtime>;
	type System = Runtime;
	type Solver = SequentialPhragmen<AccountId, Perbill>;
	type DataProvider = Staking;
	type WeightInfo = ();
	type MaxWinners = MaxWinners;
	type VotersBound = ();
	type TargetsBound = TargetsBound;
}

parameter_types! {
	pub const SessionsPerEra: SessionIndex = SESSIONS_PER_ERA;
	pub const SlashDeferDuration: EraIndex = SLASH_DEFER_DURATION;
	pub const MaxNominatorRewardedPerValidator: u32 = MAX_NOMINATORS_REWARDED_PER_VALIDATOR;
	pub const BondingDuration: EraIndex = BONDING_DURATION;
	pub const MaxNominations: u32 = MAX_NOMINATIONS;
	pub const HistoryDepth: u32 = HISTORY_DEPTH;
	pub const OffendingValidatorsThreshold: Perbill = OFFENDING_VALIDATORS_THRESHOLD;
	pub const MaxUnlockingChunks: u32 = MAX_UNLOCKING_CHUNKS;
	pub const Accuracy: Perbill = Perbill::from_percent(95);
	pub const MaxOnChainElectingVoters: u32 = MAX_ON_CHAIN_ELECTING_VOTERS;
	pub const MaxOnChainElectableTargets: u32 = MAX_ON_CHAIN_ELECTABLE_TARGETS;
}
pub struct StakingBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxNominators = ConstU32<1000>;
	type MaxValidators = ConstU32<1000>;
}
pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Runtime;
	type Solver = SequentialPhragmen<
		AccountId,
		Perbill
		// pallet_election_provider_multi_phase::SolutionAccuracyOf<Runtime>
	>;
	type DataProvider = Staking;
	type WeightInfo = frame_election_provider_support::weights::SubstrateWeight<Runtime>;
	type MaxWinners = <Runtime as pallet_elections_phragmen::Config>::DesiredMembers;
	type VotersBound = MaxOnChainElectingVoters;
	type TargetsBound = MaxOnChainElectableTargets;
}

impl pallet_staking::Config for Runtime {
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type MaxNominations = MaxNominations;
	type UnixTime = Timestamp;
	type CurrencyToVote = U128CurrencyToVote;
	type ElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type GenesisElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	type RewardRemainder = Treasury;
	type RuntimeEvent = RuntimeEvent;
	type Slash = Treasury;
	type Reward = pallet_d9_treasury::RewardBalancer<Runtime, ()>;
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	type SessionInterface = Self;
	type EraPayout = ();
	type NextNewSession = ();
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<AccountId>;
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	type VoterList = pallet_staking::UseNominatorsAndValidatorsMap<Runtime>;
	type TargetList = pallet_staking::UseValidatorsMap<Runtime>;
	type MaxUnlockingChunks = MaxUnlockingChunks;
	type OnStakerSlash = ();
	type HistoryDepth = HistoryDepth;
	type BenchmarkingConfig = StakingBenchmarkingConfig;
}

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}

parameter_types! {
	pub const Period: BlockNumber = SESSION_PERIOD;
	pub const Offset: BlockNumber = SESSION_OFFSET;
}
type PeriodicSessions = pallet_session::PeriodicSessions<Period, Offset>;
type SessionManager = pallet_session::historical::NoteHistoricalRoot<Runtime, Staking>;

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type ShouldEndSession = PeriodicSessions;
	type NextSessionRotation = PeriodicSessions;
	type SessionManager = SessionManager;
	type SessionHandler =
		<opaque::SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = opaque::SessionKeys;
	type WeightInfo = ();
}

parameter_types! {
	//note - MotionDuration constant may need to be changed to correspond to some other value, for now it will be 1 HOUR (hour in block time based on corresponding Blocks per hour )
	pub const MotionDuration: BlockNumber = HOUR;
	pub const MaxMembers: u32 = 100;
}
impl pallet_collective::Config for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = MotionDuration;
	type MaxProposals = ();
	type MaxMembers = MaxMembers;
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type WeightInfo = ();
	type SetMembersOrigin = EnsureRoot<Self::AccountId>;
	type MaxProposalWeight = ();
}

parameter_types! {
	pub const ElectionPalletId: LockIdentifier = *b"election";
	pub const CandidacyBond: Balance = CANDIDACY_BOND;
	pub const VotingBondBase: Balance = VOTING_BOND_BASE;
	pub const VotingBondFactor: Balance = VOTING_BOND_FACTOR;
	pub const DesiredMembers: u32 = DESIRED_MEMBERS;
	pub const DesiredRunnersUp: u32 = DESIRED_RUNNERS_UP;
	pub const TermDuration: BlockNumber = SESSION_PERIOD;
	pub const MaxCandidates: u32 = MAX_CANDIDATES;
	pub const MaxVotesPerVoter: u32 = MAX_VOTES_PER_VOTER;
}

impl pallet_elections_phragmen::Config for Runtime {
	type RuntimeEvent = RuntimeEvent; // Defines the event type for the runtime, which includes events from all pallets.
	type PalletId = ElectionPalletId; // The unique identifier for this pallet, used for creating unique storage keys.
	type Currency = Balances; // The currency used for transactions within this pallet (like candidacy bonds).
	type ChangeMembers = Collective; // The type which should be informed of changes to the set of elected members.
	type InitializeMembers = Collective; // The type that sets the initial membership set, usually implemented by the session manager.
	type CurrencyToVote = U128CurrencyToVote; // Used for converting balances to a vote weight for nuanced voting algorithms.
	type CandidacyBond = CandidacyBond; // The amount of currency to be locked up for submitting a candidacy.
	type VotingBondBase = VotingBondBase; // The base amount of currency to be locked up for being allowed to vote.
	type VotingBondFactor = VotingBondFactor; // A factor multiplied with the number of votes to derive the final amount of currency to be locked up for voting.
	type LoserCandidate = (); // The trait called when a candidate does not get elected.
	type KickedMember = (); // The trait called when a member gets kicked out.
	type DesiredMembers = DesiredMembers;
	type DesiredRunnersUp = DesiredRunnersUp;
	type TermDuration = TermDuration; // Defines how long each round (or "term") should last.
	type MaxCandidates = MaxCandidates; // The maximum number of candidates that can be registered for an election round.
	type MaxVoters = ();
	type MaxVotesPerVoter = MaxVotesPerVoter;
	type WeightInfo = (); // Weights for this pallet's functions. TODO[epic=staking,seq=292] Staking WeightInfo
}
parameter_types! {
	pub const MaxSpendPerTransaction: Balance = 1 * ONE_MILLION_D9_TOKENS;
}
impl pallet_d9_treasury::Config for Runtime {
	type Balance = Balance;
	type MaxSpendPerTransaction = MaxSpendPerTransaction;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
}
parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(10);
	pub const ProposalBondMinimum: Balance = ONE_THOUSAND_D9_TOKENS;
	pub const TreasuryPalletId: PalletId = PalletId(*b"treasury");
	pub const Burn: Permill = Permill::from_percent(0);
	pub const SpendPeriod: BlockNumber = 1;
}
//todo[epic=WeightInfo] manage the weightinfo, research and implement properly all that shit for the runtime pallets
impl pallet_treasury::Config for Runtime {
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type ApproveOrigin = pallet_d9_treasury::EnsureTreasurer<Runtime, ()>;
	type RejectOrigin = pallet_d9_treasury::EnsureTreasurer<Runtime, ()>;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ();
	type PalletId = TreasuryPalletId;
	type Burn = Burn;
	type SpendPeriod = SpendPeriod;
	type BurnDestination = ();
	type WeightInfo = ();
	type SpendFunds = ();
	type MaxApprovals = ();
	type SpendOrigin = pallet_d9_treasury::EnsureTreasurerLimit<Runtime, ()>;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub struct Runtime
	where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Timestamp: pallet_timestamp,
		Babe: pallet_babe,
		Grandpa: pallet_grandpa,
      Offences: pallet_offences,
      Authorship: pallet_authorship,
      ImOnline: pallet_im_online,
      AuthorityDiscovery: pallet_authority_discovery,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,
      Session: pallet_session,
      Staking:pallet_staking,
      PhragmenElections: pallet_elections_phragmen,
      Collective: pallet_collective,
      Treasury: pallet_treasury,
      D9Treasury: pallet_d9_treasury,
      Historical: pallet_session::historical::{Pallet}
	}
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<
	Address,
	RuntimeCall,
	Signature,
	SignedExtra
>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem
>;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[frame_benchmarking, BaselineBench::<Runtime>]
		[frame_system, SystemBench::<Runtime>]
		[pallet_balances, Balances]
		[pallet_timestamp, Timestamp]
		// [pallet_template, TemplateModule]
	);
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> sp_std::vec::Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
		fn authorities() -> Vec<AuthorityDiscoveryId> {
			AuthorityDiscovery::authorities()
		}
	}
   ///Grandpa API
	impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> sp_consensus_grandpa::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: sp_consensus_grandpa::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Grandpa::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}

		fn generate_key_ownership_proof(
			_set_id: sp_consensus_grandpa::SetId,
			authority_id: GrandpaId,
		) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
			use codec::Encode;

			Historical::prove((sp_consensus_grandpa::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(sp_consensus_grandpa::OpaqueKeyOwnershipProof::new)
		}
	}


	impl sp_consensus_babe::BabeApi<Block> for Runtime {
		fn configuration() -> sp_consensus_babe::BabeConfiguration {
			let epoch_config = Babe::epoch_config().unwrap_or(BABE_GENESIS_EPOCH_CONFIG);
			sp_consensus_babe::BabeConfiguration {
				slot_duration: Babe::slot_duration(),
				epoch_length: EpochDuration::get(),
				c: epoch_config.c,
				authorities: Babe::authorities().to_vec(),
				randomness: Babe::randomness(),
				allowed_slots: epoch_config.allowed_slots,
			}
		}

		fn current_epoch_start() -> sp_consensus_babe::Slot {
			Babe::current_epoch_start()
		}

		fn current_epoch() -> sp_consensus_babe::Epoch {
			Babe::current_epoch()
		}

		fn next_epoch() -> sp_consensus_babe::Epoch {
			Babe::next_epoch()
		}

		fn generate_key_ownership_proof(
			_slot: sp_consensus_babe::Slot,
			authority_id: sp_consensus_babe::AuthorityId,
		) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
			use codec::Encode;

			Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(sp_consensus_babe::OpaqueKeyOwnershipProof::new)
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
			key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Babe::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
		fn account_nonce(account: AccountId) -> Nonce {
			System::account_nonce(account)
		}
	}

	// impl assets_api::AssetsApi<
	// 	Block,
	// 	AccountId,
	// 	Balance,
	// 	u32,
	// > for Runtime
	// {
	// 	fn account_balances(account: AccountId) -> Vec<(u32, Balance)> {
	// 		Assets::account_balances(account)
	// 	}
	// }

	// impl pallet_contracts::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash, EventRecord> for Runtime
	// {
	// 	fn call(
	// 		origin: AccountId,
	// 		dest: AccountId,
	// 		value: Balance,
	// 		gas_limit: Option<Weight>,
	// 		storage_deposit_limit: Option<Balance>,
	// 		input_data: Vec<u8>,
	// 	) -> pallet_contracts_primitives::ContractExecResult<Balance, EventRecord> {
	// 		let gas_limit = gas_limit.unwrap_or(RuntimeBlockWeights::get().max_block);
	// 		Contracts::bare_call(
	// 			origin,
	// 			dest,
	// 			value,
	// 			gas_limit,
	// 			storage_deposit_limit,
	// 			input_data,
	// 			pallet_contracts::DebugInfo::UnsafeDebug,
	// 			pallet_contracts::CollectEvents::UnsafeCollect,
	// 			pallet_contracts::Determinism::Enforced,
	// 		)
	// 	}

	// 	fn instantiate(
	// 		origin: AccountId,
	// 		value: Balance,
	// 		gas_limit: Option<Weight>,
	// 		storage_deposit_limit: Option<Balance>,
	// 		code: pallet_contracts_primitives::Code<Hash>,
	// 		data: Vec<u8>,
	// 		salt: Vec<u8>,
	// 	) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, Balance, EventRecord>
	// 	{
	// 		let gas_limit = gas_limit.unwrap_or(RuntimeBlockWeights::get().max_block);
	// 		Contracts::bare_instantiate(
	// 			origin,
	// 			value,
	// 			gas_limit,
	// 			storage_deposit_limit,
	// 			code,
	// 			data,
	// 			salt,
	// 			pallet_contracts::DebugInfo::UnsafeDebug,
	// 			pallet_contracts::CollectEvents::UnsafeCollect,
	// 		)
	// 	}

	// 	fn upload_code(
	// 		origin: AccountId,
	// 		code: Vec<u8>,
	// 		storage_deposit_limit: Option<Balance>,
	// 		determinism: pallet_contracts::Determinism,
	// 	) -> pallet_contracts_primitives::CodeUploadResult<Hash, Balance>
	// 	{
	// 		Contracts::bare_upload_code(
	// 			origin,
	// 			code,
	// 			storage_deposit_limit,
	// 			determinism,
	// 		)
	// 	}

	// 	fn get_storage(
	// 		address: AccountId,
	// 		key: Vec<u8>,
	// 	) -> pallet_contracts_primitives::GetStorageResult {
	// 		Contracts::get_storage(
	// 			address,
	// 			key
	// 		)
	// 	}
	// }

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		Balance,
	> for Runtime {
		fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	// impl pallet_asset_conversion::AssetConversionApi<
	// 	Block,
	// 	Balance,
	// 	u128,
	// 	NativeOrAssetId<u32>
	// > for Runtime
	// {
	// 	fn quote_price_exact_tokens_for_tokens(asset1: NativeOrAssetId<u32>, asset2: NativeOrAssetId<u32>, amount: u128, include_fee: bool) -> Option<Balance> {
	// 		AssetConversion::quote_price_exact_tokens_for_tokens(asset1, asset2, amount, include_fee)
	// 	}

	// 	fn quote_price_tokens_for_exact_tokens(asset1: NativeOrAssetId<u32>, asset2: NativeOrAssetId<u32>, amount: u128, include_fee: bool) -> Option<Balance> {
	// 		AssetConversion::quote_price_tokens_for_exact_tokens(asset1, asset2, amount, include_fee)
	// 	}

	// 	fn get_reserves(asset1: NativeOrAssetId<u32>, asset2: NativeOrAssetId<u32>) -> Option<(Balance, Balance)> {
	// 		AssetConversion::get_reserves(&asset1, &asset2).ok()
	// 	}
	// }

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
		for Runtime
	{
		fn query_call_info(call: RuntimeCall, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_call_info(call, len)
		}
		fn query_call_fee_details(call: RuntimeCall, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_call_fee_details(call, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}
	

	// impl pallet_mmr::primitives::MmrApi<
	// 	Block,
	// 	mmr::Hash,
	// 	BlockNumber,
	// > for Runtime {
	// 	fn mmr_root() -> Result<mmr::Hash, mmr::Error> {
	// 		Ok(Mmr::mmr_root())
	// 	}

	// 	fn mmr_leaf_count() -> Result<mmr::LeafIndex, mmr::Error> {
	// 		Ok(Mmr::mmr_leaves())
	// 	}

	// 	fn generate_proof(
	// 		block_numbers: Vec<BlockNumber>,
	// 		best_known_block_number: Option<BlockNumber>,
	// 	) -> Result<(Vec<mmr::EncodableOpaqueLeaf>, mmr::Proof<mmr::Hash>), mmr::Error> {
	// 		Mmr::generate_proof(block_numbers, best_known_block_number).map(
	// 			|(leaves, proof)| {
	// 				(
	// 					leaves
	// 						.into_iter()
	// 						.map(|leaf| mmr::EncodableOpaqueLeaf::from_leaf(&leaf))
	// 						.collect(),
	// 					proof,
	// 				)
	// 			},
	// 		)
	// 	}

	// 	fn verify_proof(leaves: Vec<mmr::EncodableOpaqueLeaf>, proof: mmr::Proof<mmr::Hash>)
	// 		-> Result<(), mmr::Error>
	// 	{
	// 		let leaves = leaves.into_iter().map(|leaf|
	// 			leaf.into_opaque_leaf()
	// 			.try_decode()
	// 			.ok_or(mmr::Error::Verify)).collect::<Result<Vec<mmr::Leaf>, mmr::Error>>()?;
	// 		Mmr::verify_leaves(leaves, proof)
	// 	}

	// 	fn verify_proof_stateless(
	// 		root: mmr::Hash,
	// 		leaves: Vec<mmr::EncodableOpaqueLeaf>,
	// 		proof: mmr::Proof<mmr::Hash>
	// 	) -> Result<(), mmr::Error> {
	// 		let nodes = leaves.into_iter().map(|leaf|mmr::DataOrHash::Data(leaf.into_opaque_leaf())).collect();
	// 		pallet_mmr::verify_leaves_proof::<mmr::Hashing, _>(root, nodes, proof)
	// 	}
	// }

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade(checks).unwrap();
			(weight, RuntimeBlockWeights::get().max_block)
		}

		fn execute_block(
			block: Block,
			state_root_check: bool,
			signature_check: bool,
			select: frame_try_runtime::TryStateSelect
		) -> Weight {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
		}
	}

	// impl pallet_staking_runtime_api::StakingApi<Block, Balance> for Runtime {
	// 	fn nominations_quota(balance: Balance) -> u32 {
	// 		Staking::api_nominations_quota(balance)
	// 	}
	// }

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;

			// Trying to add benchmarks directly to the Session Pallet caused cyclic dependency
			// issues. To get around that, we separated the Session benchmarks into its own crate,
			// which is why we need these two lines below.
			use pallet_session_benchmarking::Pallet as SessionBench;
			use pallet_offences_benchmarking::Pallet as OffencesBench;
			use pallet_election_provider_support_benchmarking::Pallet as EPSBench;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;
			use pallet_nomination_pools_benchmarking::Pallet as NominationPoolsBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch,  TrackedStorageKey};

			// Trying to add benchmarks directly to the Session Pallet caused cyclic dependency
			// issues. To get around that, we separated the Session benchmarks into its own crate,
			// which is why we need these two lines below.
			use pallet_session_benchmarking::Pallet as SessionBench;
			use pallet_offences_benchmarking::Pallet as OffencesBench;
			use pallet_election_provider_support_benchmarking::Pallet as EPSBench;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;
			use pallet_nomination_pools_benchmarking::Pallet as NominationPoolsBench;

			impl pallet_session_benchmarking::Config for Runtime {}
			impl pallet_offences_benchmarking::Config for Runtime {}
			impl pallet_election_provider_support_benchmarking::Config for Runtime {}
			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}
			impl pallet_nomination_pools_benchmarking::Config for Runtime {}

			use frame_support::traits::WhitelistedStorageKeys;
			let mut whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

			// Treasury Account
			// TODO: this is manual for now, someday we might be able to use a
			// macro for this particular key
			let treasury_key = frame_system::Account::<Runtime>::hashed_key_for(Treasury::account_id());
			whitelist.push(treasury_key.to_vec().into());

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);
			Ok(batches)
		}
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::traits::WhitelistedStorageKeys;
	use sp_core::hexdisplay::HexDisplay;
	use std::collections::HashSet;

	#[test]
	fn check_whitelist() {
		let whitelist: HashSet<String> = AllPalletsWithSystem::whitelisted_storage_keys()
			.iter()
			.map(|e| HexDisplay::from(&e.key).to_string())
			.collect();

		// Block Number
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac")
		);
		// Total Issuance
		assert!(
			whitelist.contains("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80")
		);
		// Execution Phase
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a")
		);
		// Event Count
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850")
		);
		// System Events
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7")
		);
	}
}
