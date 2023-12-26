#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
pub mod constants;
use sp_core::crypto::UncheckedFrom;
pub use crate::constants::*;
use frame_system::EnsureRoot;
use frame_support::traits::AsEnsureOriginWithArg;
use frame_support::PalletId;
use frame_support::pallet_prelude::{ Encode, Decode, RuntimeDebug };
use pallet_grandpa::AuthorityId as GrandpaId;
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_staking::{ EraIndex, SessionIndex };
use sp_core::{ crypto::KeyTypeId, OpaqueMetadata };
use sp_runtime::DispatchError;
use frame_support::log::error;
#[cfg(feature = "runtime-benchmarks")]
use pallet_contracts::NoopMigration;
use pallet_contracts::chain_extension::{
	ChainExtension,
	Environment,
	Ext,
	InitState,
	RetVal,
	SysConfig,
};
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
	transaction_validity::{ TransactionSource, TransactionValidity },
	ApplyExtrinsicResult,
	MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
// use frame_election_provider_support::{ onchain, SequentialPhragmen };
// A few exports that help ease life for downstream crates.

pub use frame_support::{
	construct_runtime,
	parameter_types,
	traits::{
		ConstU128,
		ConstU32,
		ConstU64,
		ConstU8,
		KeyOwnerProofSystem,
		Randomness,
		StorageInfo,
		Nothing,
		ConstBool,
	},
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
pub use pallet_d9_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ ConstFeeMultiplier, CurrencyAdapter, Multiplier };
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{ Perbill, Permill, transaction_validity::TransactionPriority };
/// Import the template pallet.
// pub use pallet_template;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;
/// Index of a transaction in the chain.
pub type Index = u32;
/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

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
			pub aura: Aura,
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
	pub const SS58Prefix: u8 = 9;
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
	type AccountData = pallet_d9_balances::AccountData<Balance>;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The set code logic, just the default since we're not a parachain.
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<27>;
}

impl pallet_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;

	type WeightInfo = ();
	type MaxAuthorities = ConstU32<27>;
	type MaxSetIdSessionEntries = ConstU64<0>;

	type KeyOwnerProof = sp_core::Void;
	type EquivocationReportSystem = ();
}
parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}
impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}
parameter_types! {
	pub const MaxReferralDepth: u32 = 19;
}
impl pallet_d9_referral::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MaxReferralDepth = MaxReferralDepth;
	type SetMaxReferralDepthOrigin = pallet_collective::EnsureProportionAtLeast<
		AccountId,
		(),
		1,
		2
	>;
}

impl pallet_d9_balances::ReferralManager<Runtime, ()> for Runtime {
	fn get_parent(account: &AccountId) -> Option<AccountId> {
		pallet_d9_referral::Pallet::<Runtime>::get_parent(account)
	}

	fn create_referral_relationship(parent: &AccountId, child: &AccountId) {
		let _ = pallet_d9_referral::Pallet::<Runtime>::create_referral_relationship(parent, child);
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
pub struct SomeIdentifier(pub [u8; 4]);
parameter_types! {
	pub const MaxLocks: u32 = 50;
	pub const ExistentialDeposit: u128 = EXISTENTIAL_DEPOSIT;
	pub const ReserveIdentifier: SomeIdentifier = SomeIdentifier(*b"rsrv");
	pub const FreezeIdentifier: SomeIdentifier = SomeIdentifier(*b"frze");
	pub const MaxHolds: u32 = 50;
	pub const MaxReserves: u32 = 50;
	pub const MaxFreezes: u32 = 50;
	pub const HoldIdentifier: SomeIdentifier = SomeIdentifier(*b"hold");
}
impl pallet_d9_balances::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_d9_balances::weights::SubstrateWeight<Runtime>;
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
	type ReferralManager = Self;
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
	pub const RemoveItemsLimit: u32 = 1000;
	pub const AssetDeposit: Balance = 10 * ONE_MILLION_D9_TOKENS;
	pub const ApprovalDeposit: Balance = 1 * D9_TOKEN;
	pub const AssetAccountDeposit: Balance = 1 * D9_TOKEN;
	pub const StringLimit: u32 = 20;
	pub const MetadataDepositBase: Balance = 10 * ONE_THOUSAND_D9_TOKENS;
	pub const MetadataDepositPerByte: Balance = 1000 * D9_TOKEN;
}
impl pallet_assets::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
	type RemoveItemsLimit = RemoveItemsLimit;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

// parameter_types! {
// 	pub const MaxWinners: u32 = DESIRED_MEMBERS;
// 	pub const TargetsBound: u32 = DESIRED_RUNNERS_UP;
// }
// // type Solver = frame_election_provider_support::SequentialPhragmen<AccountId, Perbill>;
// impl frame_election_provider_support::onchain::Config for Runtime {
// 	// type System = frame_system::Pallet<Runtime>;
// 	type System = Runtime;
// 	type Solver = SequentialPhragmen<AccountId, Perbill>;
// 	type DataProvider = Staking;
// 	type WeightInfo = ();
// 	type MaxWinners = MaxWinners;
// 	type VotersBound = ();
// 	type TargetsBound = TargetsBound;
// }

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
// pub struct StakingBenchmarkingConfig;
// impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
// 	type MaxNominators = ConstU32<1000>;
// 	type MaxValidators = ConstU32<1000>;
// }
// pub struct OnChainSeqPhragmen;
// impl onchain::Config for OnChainSeqPhragmen {
// 	type System = Runtime;
// 	type Solver = SequentialPhragmen<
// 		AccountId,
// 		Perbill
// 		// pallet_election_provider_multi_phase::SolutionAccuracyOf<Runtime>
// 	>;
// 	type DataProvider = Staking;
// 	type WeightInfo = frame_election_provider_support::weights::SubstrateWeight<Runtime>;
// 	type MaxWinners = <Runtime as pallet_elections_phragmen::Config>::DesiredMembers;
// 	type VotersBound = MaxOnChainElectingVoters;
// 	type TargetsBound = MaxOnChainElectableTargets;
// }

// impl pallet_staking::Config for Runtime {
// 	type Currency = Balances;
// 	type CurrencyBalance = Balance;
// 	type MaxNominations = MaxNominations;
// 	type UnixTime = Timestamp;
// 	type CurrencyToVote = U128CurrencyToVote;
// 	type ElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
// 	type GenesisElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
// 	type RewardRemainder = Treasury;
// 	type RuntimeEvent = RuntimeEvent;
// 	type Slash = Treasury;
// 	type Reward = pallet_d9_treasury::RewardBalancer<Runtime, ()>;
// 	type SessionsPerEra = SessionsPerEra;
// 	type BondingDuration = BondingDuration;
// 	type SlashDeferDuration = SlashDeferDuration;
// 	type SessionInterface = Self;
// 	type EraPayout = ();
// 	type NextNewSession = ();
// 	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
// 	type WeightInfo = ();
// 	type AdminOrigin = EnsureRoot<AccountId>;
// 	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
// 	type VoterList = pallet_staking::UseNominatorsAndValidatorsMap<Runtime>;
// 	type TargetList = pallet_staking::UseValidatorsMap<Runtime>;
// 	type MaxUnlockingChunks = MaxUnlockingChunks;
// 	type OnStakerSlash = ();
// 	type HistoryDepth = HistoryDepth;
// 	type BenchmarkingConfig = StakingBenchmarkingConfig;
// }

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_d9_burn_election::ValidatorStats<Runtime>;
	type FullIdentificationOf = pallet_d9_burn_election::ValidatorStatsOf<Runtime>;
}

parameter_types! {
	pub const MaxAuthorities: u32 = MAX_VALIDATOR_NODES;
}
impl pallet_authority_discovery::Config for Runtime {
	type MaxAuthorities = MaxAuthorities;
}
parameter_types! {
	pub const CurrencySubUnits: u128 = 1_000_000_000_000;
	pub const MaxCandidates: u32 = MAX_CANDIDATES;
	pub const MaxValidatorNodes: u32 = MAX_VALIDATOR_NODES;
}

impl pallet_d9_burn_election::Config for Runtime {
	type CurrencySubUnits = CurrencySubUnits;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type MaxCandidates = MaxCandidates;
	type MaxValidatorNodes = MaxValidatorNodes;
}
parameter_types! {
	pub const Period: BlockNumber = SESSION_PERIOD;
	pub const Offset: BlockNumber = SESSION_OFFSET;
}
type PeriodicSessions = pallet_session::PeriodicSessions<Period, Offset>;
// type SessionManager = pallet_session::historical::NoteHistoricalRoot<Runtime, Staking>;

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_d9_burn_election::ConvertAccountId<Self>;
	type ShouldEndSession = PeriodicSessions;
	type NextSessionRotation = PeriodicSessions;
	type SessionManager = D9BurnElection;
	type SessionHandler =
		<opaque::SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = opaque::SessionKeys;
	type WeightInfo = ();
}

parameter_types! {
	//note - MotionDuration constant may need to be changed to correspond to some other value, for now it will be 1 HOUR (hour in block time based on corresponding Blocks per hour )
	pub const MotionDuration: BlockNumber = WEEK;
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
	pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	pub const MaxKeys: u32 = MAX_VALIDATOR_NODES + 10;
	pub const MaxPeerInHeartbeats: u32 = MAX_VALIDATOR_NODES + 10;
	pub const MaxPeerDataEncodingSize: u32 = 1024;
}
impl pallet_im_online::Config for Runtime {
	type AuthorityId = pallet_im_online::sr25519::AuthorityId;
	type RuntimeEvent = RuntimeEvent;
	type NextSessionRotation = PeriodicSessions;
	type ValidatorSet = Historical;
	type ReportUnresponsiveness = ();
	type UnsignedPriority = ImOnlineUnsignedPriority;
	type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
	type MaxKeys = MaxKeys;
	type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
	type MaxPeerDataEncodingSize = MaxPeerDataEncodingSize;
}

impl<T> frame_system::offchain::SendTransactionTypes<T> for Runtime where RuntimeCall: From<T> {
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = RuntimeCall;
}
// impl pallet_offences::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
// 	type OnOffenceHandler = ();
// }

// parameter_types! {
// 	pub const ElectionPalletId: LockIdentifier = *b"election";
// 	pub const CandidacyBond: Balance = CANDIDACY_BOND;
// 	pub const VotingBondBase: Balance = VOTING_BOND_BASE;
// 	pub const VotingBondFactor: Balance = VOTING_BOND_FACTOR;
// 	pub const DesiredMembers: u32 = MAX_VALIDATOR_NODES;
// 	pub const DesiredRunnersUp: u32 = DESIRED_RUNNERS_UP;
// 	pub const TermDuration: BlockNumber = SESSION_PERIOD;
// 	pub const MaxCandidates: u32 = MAX_CANDIDATES;
// 	pub const MaxVotesPerVoter: u32 = MAX_VOTES_PER_VOTER;
// }
// impl pallet_elections_phragmen::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent; // Defines the event type for the runtime, which includes events from all pallets.
// 	type PalletId = ElectionPalletId; // The unique identifier for this pallet, used for creating unique storage keys.
// 	type Currency = Balances; // The currency used for transactions within this pallet (like candidacy bonds).
// 	type ChangeMembers = Collective; // The type which should be informed of changes to the set of elected members.
// 	type InitializeMembers = Collective; // The type that sets the initial membership set, usually implemented by the session manager.
// 	type CurrencyToVote = U128CurrencyToVote; // Used for converting balances to a vote weight for nuanced voting algorithms.
// 	type CandidacyBond = CandidacyBond; // The amount of currency to be locked up for submitting a candidacy.
// 	type VotingBondBase = VotingBondBase; // The base amount of currency to be locked up for being allowed to vote.
// 	type VotingBondFactor = VotingBondFactor; // A factor multiplied with the number of votes to derive the final amount of currency to be locked up for voting.
// 	type LoserCandidate = (); // The trait called when a candidate does not get elected.
// 	type KickedMember = (); // The trait called when a member gets kicked out.
// 	type DesiredMembers = DesiredMembers;
// 	type DesiredRunnersUp = DesiredRunnersUp;
// 	type TermDuration = TermDuration; // Defines how long each round (or "term") should last.
// 	type MaxCandidates = MaxCandidates; // The maximum number of candidates that can be registered for an election round.
// 	type MaxVoters = ();
// 	type MaxVotesPerVoter = MaxVotesPerVoter;
// 	type WeightInfo = (); // Weights for this pallet's functions. TODO[epic=staking,seq=292] Staking WeightInfo
// }

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

impl pallet_treasury::Config for Runtime {
	type ApproveOrigin = pallet_d9_treasury::EnsureTreasurer<Runtime, ()>;
	type Burn = Burn;
	type BurnDestination = ();
	type Currency = Balances;
	type MaxApprovals = ();
	type OnSlash = Treasury;
	type PalletId = TreasuryPalletId;
	type ProposalBond = ProposalBond;
	type ProposalBondMaximum = ();
	type ProposalBondMinimum = ProposalBondMinimum;
	type RejectOrigin = pallet_d9_treasury::EnsureTreasurer<Runtime, ()>;
	type RuntimeEvent = RuntimeEvent;
	type SpendFunds = ();
	type SpendOrigin = pallet_d9_treasury::EnsureTreasurerLimit<Runtime, ()>;
	type SpendPeriod = SpendPeriod;
	type WeightInfo = ();
}

impl pallet_insecure_randomness_collective_flip::Config for Runtime {}
#[derive(Default)]
pub struct D9ChainExtension;

impl ChainExtension<Runtime> for D9ChainExtension {
	fn call<E>(
		&mut self,
		env: Environment<E, InitState>
	)
		-> pallet_contracts::chain_extension::Result<RetVal>
		where
			E: Ext<T = Runtime>,
			<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>
	{
		let mut env = env.buf_in_buf_out();
		let func_id = env.func_id();

		match func_id {
			0 => {
				let account: AccountId = env.read_as()?;
				let parent = pallet_d9_referral::Pallet::<Runtime>::get_parent(&account);
				let parent_bytes = parent.encode();
				let _ = env.write(&parent_bytes, false, None);
			}
			1 => {
				let account = env.read_as()?;
				let ancestors = pallet_d9_referral::Pallet::<Runtime>::get_ancestors(account);
				let ancestors_bytes = ancestors.encode();
				let _ = env.write(&ancestors_bytes, false, None);
			}
			_ => {
				error!("Called an unregistered `func_id`: {:}", func_id);
				return Err(DispatchError::Other("Unimplemented func_id"));
			}
		}
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		true
	}
}
parameter_types! {
	pub const DepositPerItem: Balance = DEPOSIT_PER_ITEM   ;
	pub const DepositPerByte: Balance = DEPOSIT_PER_BYTE;
	pub const DefaultDepositLimit: Balance = DEFAULT_DEPOSIT_LIMIT;
	pub Schedule: pallet_contracts::Schedule<Runtime> = Default::default();
   pub const MaxCodeLen: u32 = MAX_CODE_SIZE;
   pub const MaxDebugBufferLen:u32 = MAX_DEBUG_BUFFER_LENGTH;
   pub const MaxStorageKeyLen:u32 = MAX_STORAGE_KEY_LENGTH;
}
impl pallet_contracts::Config for Runtime {
	type Time = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	/// The safest default is to allow no calls at all.
	///
	/// Runtimes should whitelist dispatchables that are allowed to be called from contracts
	/// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
	/// change because that would break already deployed contracts. The `Call` structure itself
	/// is not allowed to change the indices of existing pallets, too.
	type CallFilter = Nothing;
	type WeightPrice = pallet_transaction_payment::Pallet<Self>;
	type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
	type ChainExtension = D9ChainExtension;
	type Schedule = Schedule;
	type CallStack = [pallet_contracts::Frame<Self>; 5];
	type DepositPerByte = DepositPerByte;
	type DefaultDepositLimit = DefaultDepositLimit;
	type DepositPerItem = DepositPerItem;
	type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
	type MaxCodeLen = MaxCodeLen;
	type MaxStorageKeyLen = MaxStorageKeyLen;
	type UnsafeUnstableInterface = ConstBool<false>;
	type MaxDebugBufferLen = MaxDebugBufferLen;
	// #[cfg(not(feature = "runtime-benchmarks"))]
	// type Migrations = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Migrations = (NoopMigration<1>, NoopMigration<2>);
}

parameter_types! {
	pub const DepositBase: Balance = 1 * D9_TOKEN;
	pub const DepositFactor: Balance = 1 * D9_TOKEN;
	pub const MaxSignatories: u32 = 100;
}
impl pallet_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;
}
// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub struct Runtime
	where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		Aura: pallet_aura,
      D9Referral:pallet_d9_referral,
      D9Treasury: pallet_d9_treasury,
      D9BurnElection:pallet_d9_burn_election,
		Balances: pallet_d9_balances,
		Grandpa: pallet_grandpa,
		MultiSig: pallet_multisig,
		Sudo: pallet_sudo,
		System: frame_system,
		Timestamp: pallet_timestamp,
		TransactionPayment: pallet_transaction_payment,
      Assets:pallet_assets,
      AuthorityDiscovery: pallet_authority_discovery,
      Collective: pallet_collective,
      Contracts:pallet_contracts,
      Historical: pallet_session::historical,
      ImOnline: pallet_im_online,
      // Offences: pallet_offences,
      // PhragmenElections: pallet_elections_phragmen,
      RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip,
      Session: pallet_session,
      // Staking:pallet_staking,
      Treasury: pallet_treasury,
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
		[pallet_d9_balances, Balances]
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

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
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

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

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

	impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> sp_consensus_grandpa::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			_equivocation_proof: sp_consensus_grandpa::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			_key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			None
		}

		fn generate_key_ownership_proof(
			_set_id: sp_consensus_grandpa::SetId,
			_authority_id: GrandpaId,
		) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
			// SUBSTRATE_NOTE: this is the only implementation possible since we've
			// defined our key owner proof type as a bottom type (i.e. a type
			// with no values).
			None
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
		for Runtime
	{
		fn query_call_info(
			call: RuntimeCall,
			len: u32,
		) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_call_info(call, len)
		}
		fn query_call_fee_details(
			call: RuntimeCall,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_call_fee_details(call, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;
         use frame_support::PalletId;
			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			use frame_support::traits::WhitelistedStorageKeys;
			let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);

			Ok(batches)
		}


	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
			// SUBSTRATE_NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade(checks).unwrap();
			(weight, BlockWeights::get().max_block)
		}

		fn execute_block(
			block: Block,
			state_root_check: bool,
			signature_check: bool,
			select: frame_try_runtime::TryStateSelect
		) -> Weight {
			// SUBSTRATE_NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, signature_check, select).expect("execute-block failed")
		}
	}

impl pallet_contracts::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash> for Runtime
	{
		fn call(
			origin: AccountId,
			dest: AccountId,
			value: Balance,
			gas_limit: Option<Weight>,
			storage_deposit_limit: Option<Balance>,
			input_data: Vec<u8>,
		) -> pallet_contracts_primitives::ContractExecResult<Balance> {
			let gas_limit = gas_limit.unwrap_or(BlockWeights::get().max_block);
			Contracts::bare_call(
				origin,
				dest,
				value,
				gas_limit,
				storage_deposit_limit,
				input_data,
				true,
				pallet_contracts::Determinism::Enforced,
			)
		}

		fn instantiate(
			origin: AccountId,
			value: Balance,
			gas_limit: Option<Weight>,
			storage_deposit_limit: Option<Balance>,
			code: pallet_contracts_primitives::Code<Hash>,
			data: Vec<u8>,
			salt: Vec<u8>,
		) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, Balance>
		{
			let gas_limit = gas_limit.unwrap_or(BlockWeights::get().max_block);
			Contracts::bare_instantiate(
				origin,
				value,
				gas_limit,
				storage_deposit_limit,
				code,
				data,
				salt,
				true
			)
		}

		fn upload_code(
			origin: AccountId,
			code: Vec<u8>,
			storage_deposit_limit: Option<Balance>,
			determinism: pallet_contracts::Determinism,
		) -> pallet_contracts_primitives::CodeUploadResult<Hash, Balance>
		{
			Contracts::bare_upload_code(
				origin,
				code,
				storage_deposit_limit,
				determinism,
			)
		}

		fn get_storage(
			address: AccountId,
			key: Vec<u8>,
		) -> pallet_contracts_primitives::GetStorageResult {
			Contracts::get_storage(
				address,
				key
			)
		}
	}
      impl runtime_api::ReferralRuntimeApi<Block, AccountId> for Runtime{
      fn get_parent(account: AccountId) -> Option<AccountId> {
         pallet_d9_referral::Pallet::<Runtime>::get_parent(&account)
      }
      fn get_ancestors(account:AccountId)->Option<Vec<AccountId>>{
         pallet_d9_referral::Pallet::<Runtime>::get_ancestors(account)
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
