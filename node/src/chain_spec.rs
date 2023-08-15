use d9_node_runtime::{
	AccountId,
	BabeConfig,
	BalancesConfig,
	GenesisConfig,
	GrandpaConfig,
	Signature,
	SudoConfig,
	SessionConfig,
	StakingConfig,
	PhragmenElectionsConfig,
	D9TreasuryConfig,
	SystemConfig,
	WASM_BINARY,
	CollectiveConfig,
	TreasuryConfig,
	AuthorityDiscoveryConfig,
};
use sc_service::ChainType;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{ sr25519, Pair, Public };
use sp_runtime::traits::{ IdentifyAccount, Verify };
use sp_runtime::Perbill;
// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
	where AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(
		ChainSpec::from_genesis(
			// Name
			"dev_d9",
			// ID
			"dev_d9",
			ChainType::Development,
			move || {
				testnet_genesis(
					wasm_binary,
					// Initial PoA authorities
					vec![authority_keys_from_seed("Alice")],
					// Sudo account
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					// Pre-funded accounts
					vec![
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						get_account_id_from_seed::<sr25519::Public>("Bob//stash")
					],
					true
				)
			},
			// Bootnodes
			vec![],
			// Telemetry
			None,
			// Protocol ID
			Some("dev_d9"),
			Some("dev_d9_original"),
			// Properties
			None,
			// Extensions
			None
		)
	)
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(
		ChainSpec::from_genesis(
			// Name
			"local_d9",
			// ID
			"local_d9",
			ChainType::Local,
			move || {
				testnet_genesis(
					wasm_binary,
					// Initial PoA authorities
					vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
					// Sudo account
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					// Pre-funded accounts
					vec![
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash")
					],
					true
				)
			},
			// Bootnodes
			vec![],
			// Telemetry
			None,
			// Protocol ID
			None,
			// Properties
			None,
			None,
			// Extensions
			None
		)
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(BabeId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1 << 60))
				.collect(),
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(d9_node_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities
				.iter()
				.map(|x| (x.1.clone(), 1))
				.collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key.clone()),
		},
		session: SessionConfig {
			..Default::default()
			// keys: initial_authorities
			// 	.iter()
			// 	.map(|x| (x.0.clone(), x.0.clone()))
			// 	.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: 27,
			minimum_validator_count: 1,
			invulnerables: vec![],
			slash_reward_fraction: Perbill::from_percent(20),
			..Default::default()
		},
		phragmen_elections: PhragmenElectionsConfig {
			members: vec![],
		},
		d9_treasury: D9TreasuryConfig {
			treasurer: Some(root_key.clone()),
			..Default::default()
		},
		transaction_payment: Default::default(),
		collective: CollectiveConfig {
			members: vec![],
			phantom: Default::default(),
		},
		treasury: TreasuryConfig {
			..Default::default()
		},
		im_online: ImOnlineConfig {
			keys: vec![],
		},
		authority_discovery: AuthorityDiscoveryConfig {
			keys: vec![],
		},
	}
}
