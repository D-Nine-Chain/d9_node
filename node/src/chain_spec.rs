use d9_node_runtime::{
	AccountId,
	AssetsConfig,
	AuraConfig,
	AuthorityDiscoveryConfig,
	BalancesConfig,
	D9ReferralConfig,
	CollectiveConfig,
	D9TreasuryConfig,
	D9NodeVotingConfig,
	GenesisConfig,
	GrandpaConfig,
	ImOnlineConfig,
	// PhragmenElectionsConfig,
	SessionConfig,
	Signature,
	// StakingConfig,
	SudoConfig,
	SystemConfig,
	TreasuryConfig,
	WASM_BINARY,
	opaque::SessionKeys,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{ sr25519, Pair, Public };
use sp_runtime::traits::{ IdentifyAccount, Verify };
// use sp_runtime::Perbill;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sc_chain_spec::Properties;
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

/// Generates authority keys for various Substrate services from a given seed.
///
/// This function is a utility to easily generate keys for different authority roles
/// in a Substrate-based chain. The generated keys are determined by the provided seed
/// and are crucial for the operation of consensus, online presence reporting, and authority discovery.
///
/// # Arguments
///
/// * `s`: A seed string that is used as a base to deterministically generate the keys.
///
/// # Returns
///
/// A tuple containing keys for the following services, in order:
/// - `AuraId`: The key for the AURA consensus algorithm.
/// - `GrandpaId`: The key for the GRANDPA finality gadget.
/// - `ImOnlineId`: The key for the I'm Online module, used to report online presence.
/// - `AuthorityDiscoveryId`: The key for the Authority Discovery service, aiding in
///   network-related tasks for validators.
///
/// # Examples
///
/// ```ignore
/// let seed = "my_unique_seed";
/// let keys = authority_keys_from_seed(seed);
/// println!("AuraId: {:?}", keys.0);
/// ```
///
/// # Note
///
/// The determinism of the generated keys is based on the provided seed and the
/// cryptographic functions employed by `get_from_seed`. Ensure a secure and unique
/// seed for actual usage in a live environment.
pub fn authority_keys_from_seed(
	s: &str
) -> (AccountId, AuraId, GrandpaId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(s),
		// get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
		get_from_seed::<AuthorityDiscoveryId>(s),
	)
}

fn session_keys(
	aura: AuraId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId
) -> SessionKeys {
	SessionKeys { aura, grandpa, im_online, authority_discovery }
}
pub fn live_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Main wasm not available".to_string())?;
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "D9".into());
	properties.insert("tokenDecimals".into(), (12).into());
	properties.insert("ss58Format".into(), (9).into());
	Ok(
		ChainSpec::from_genesis(
			// Name
			"D9",
			// ID
			"d9_main",
			ChainType::Live,
			move || {
				network_genesis(
					wasm_binary,
					// Initial PoA authorities
					vec![
						authority_keys_from_seed(""),
						authority_keys_from_seed(""),
						authority_keys_from_seed("")
					],
					// Sudo account
					get_account_id_from_seed::<sr25519::Public>(""),
					// Pre-funded accounts
					vec![get_account_id_from_seed::<sr25519::Public>("")],
					true
				)
			},
			// Bootnodes
			vec![],
			// Telemetry
			None,
			// Protocol ID
			Some("D9_main"),
			//fork ID
			Some("d9_main"),
			// Properties
			Some(properties),
			// Extensions
			None
		)
	)
}
pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "D9".into());
	properties.insert("tokenDecimals".into(), (12).into());
	properties.insert("ss58Format".into(), (9).into());
	Ok(
		ChainSpec::from_genesis(
			// Name
			"dev_d9_v2",
			// ID
			"dev_d9_v2",
			ChainType::Development,
			move || {
				network_genesis(
					wasm_binary,
					// Initial PoA authorities
					vec![
						authority_keys_from_seed("Alice"),
						authority_keys_from_seed("Bob"),
						authority_keys_from_seed("Charlie")
					],
					// Sudo account
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					// Pre-funded accounts
					vec![
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_account_id_from_seed::<sr25519::Public>("Charlie")
					],
					true
				)
			},
			// Bootnodes
			vec![],
			// Telemetry
			None,
			// Protocol ID
			Some("dev_D9_v2"),
			//fork ID
			Some("dev_d9_v2"),
			// Properties
			Some(properties),
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
				network_genesis(
					wasm_binary,
					// Initial PoA authorities
					vec![
						authority_keys_from_seed("Alice"),
						authority_keys_from_seed("Bob"),
						authority_keys_from_seed("Charlie")
					],
					// Sudo account
					get_account_id_from_seed::<sr25519::Public>("Alice"),
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
fn network_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId, ImOnlineId, AuthorityDiscoveryId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool
) -> GenesisConfig {
	GenesisConfig {
		assets: AssetsConfig {
			..Default::default()
		},
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		d9_referral: D9ReferralConfig {
			..Default::default()
		},
		d9_node_voting: D9NodeVotingConfig {
			initial_candidates: initial_authorities
				.iter()
				.map(|x| x.0.clone())
				.collect(),
		},

		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 10_000_000_000_000_000_000_000))
				.collect(),
		},
		aura: AuraConfig {
			..Default::default()
			// authorities: initial_authorities
			// 	.iter()
			// 	.map(|x| x.1.clone())
			// 	.collect(),
		},
		grandpa: GrandpaConfig {
			..Default::default()
			// authorities: initial_authorities
			// 	.iter()
			// 	.map(|x| (x.2.clone(), 1))
			// 	.collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key.clone()),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.1.clone(), x.2.clone(), x.3.clone(), x.4.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		// staking: StakingConfig {
		// 	validator_count: 27,
		// 	minimum_validator_count: 1,
		// 	invulnerables: vec![],
		// 	slash_reward_fraction: Perbill::from_percent(20),
		// 	..Default::default()
		// },
		// phragmen_elections: PhragmenElectionsConfig {
		// 	members: vec![],
		// },
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
			..Default::default()
		},
		authority_discovery: AuthorityDiscoveryConfig {
			..Default::default()
			// keys: initial_authorities
			// 	.iter()
			// 	.map(|x| { x.5.clone() })
			// 	.collect::<Vec<_>>(),
		},
	}
}
