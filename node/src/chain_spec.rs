use kumandra_node_runtime::{
	opaque::SessionKeys, wasm_binary_unwrap, AccountId, AuthorityDiscoveryConfig, Balance,
	BalancesConfig, Block, CouncilConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig,
	IndicesConfig, MaxNominations, BabeConfig, SessionConfig, Signature, StakerStatus,
	StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig, DOLLARS,
};
use kmp_consensus_rrsc::AuthorityId as RRSCId;
use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_segment_book::sr25519::AuthorityId as SegmentBookId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{config::TelemetryEndpoints, ChainType};
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: kmc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

type AccountPublic = <Signature as Verify>::Signer;

fn session_keys(
	grandpa: GrandpaId,
	rrsc: RRSCId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
	segment_book: SegmentBookId,
) -> SessionKeys {
	SessionKeys { grandpa, rrsc, im_online, authority_discovery, segment_book }
}

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, RRSCId, ImOnlineId, AuthorityDiscoveryId, SegmentBookId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<RRSCId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
		get_from_seed::<SegmentBookId>(seed),
	)
}

fn kumandra_testnet_config_genesis() -> GenesisConfig {
	#[rustfmt::skip]
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	//
	// and
	//
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		RRSCId,
		ImOnlineId,
		AuthorityDiscoveryId,
		SegmentBookId,
	)> = vec![
		(
			//2 5ENzXncBvqYviMWvG7DYxvKdNhLZTHgjWNyzYNuBTL7mGAX3
			hex!["666b03f17ac057c0dfc1b3ed93f19c119a9884021918e9f7fa8c189a2e621b02"].into(),
			//1 5C8Egtiq4Hw5cA5JmcBetT4wck5ucy7otHu4LbGLwGcH2HXW
			hex!["02b1862bfc7b35281eec139d94903cac1110cedff6b82cffebada8cb7de4815b"].into(),
			//3 5ENzXncBvqYviMWvG7DYxvKdNhLZTHgjWNyzYNuBTL7mGAX3
			hex!["6ee68b505a25f50f64b4748f8e87ae3c56c4707fcea5cb53527e458a05f8644d"]
				.unchecked_into(),
			//4 5G4WoNYpjF2gyDFGRh3749tst6eKzSRqQrxMjYoAu29XBMwH
			hex!["b0cbb6938aa2aaf89c53e65e0cf1881bbaff26131121b45ea7f2270e4d20d556"]
				.unchecked_into(),
			//4 5G4WoNYpjF2gyDFGRh3749tst6eKzSRqQrxMjYoAu29XBMwH
			hex!["b0cbb6938aa2aaf89c53e65e0cf1881bbaff26131121b45ea7f2270e4d20d556"]
				.unchecked_into(),
			//4 5G4WoNYpjF2gyDFGRh3749tst6eKzSRqQrxMjYoAu29XBMwH
			hex!["b0cbb6938aa2aaf89c53e65e0cf1881bbaff26131121b45ea7f2270e4d20d556"]
				.unchecked_into(),
			hex!["b0cbb6938aa2aaf89c53e65e0cf1881bbaff26131121b45ea7f2270e4d20d556"]
				.unchecked_into(),
		),
		(
			//5 5FkPu1aGhxhpMjJSbWggrVnXs159SF3nNizfpM3h8ghJMQBd
			hex!["a2fa0f1e725892efec098cc25b17ef9214dbaee50de087bf293e0f9ddcf7b51f"].into(),
			//6 5D86FDCz3bejRBgGPFQxqHHVZFYXMRC1RiPD4F5wM2JbGYtD
			hex!["2ed1562d419c5994d9b6f59b154d9abf49b9c69c5cd0db2d8755e3e58e8bb84e"].into(),
			//7 5EvSo77NqBNaRxmxBQMTGP8TUoehPm8wZtfg9tJfcJUhi5C4
			hex!["7e68092d8db6f74e9ffd3d68d822a01387e6f157e8b14f83e752073dbbaaf242"]
				.unchecked_into(),
			//8 5HjDrQ1pm89n3FZM4q9J4jNx5ufu3MjZ7pQ6f46uwjUenRxG
			hex!["fa8d7744f7b2ac8a9c8359e012b548b3f6aba9154861b249fa2fadf405e79924"]
				.unchecked_into(),
			//8 5HjDrQ1pm89n3FZM4q9J4jNx5ufu3MjZ7pQ6f46uwjUenRxG
			hex!["fa8d7744f7b2ac8a9c8359e012b548b3f6aba9154861b249fa2fadf405e79924"]
				.unchecked_into(),
			//8 5HjDrQ1pm89n3FZM4q9J4jNx5ufu3MjZ7pQ6f46uwjUenRxG
			hex!["fa8d7744f7b2ac8a9c8359e012b548b3f6aba9154861b249fa2fadf405e79924"]
				.unchecked_into(),
			hex!["fa8d7744f7b2ac8a9c8359e012b548b3f6aba9154861b249fa2fadf405e79924"]
				.unchecked_into(),
		),
		(
			//9 5CdTvMU9ac3Gna5eHVg7H8CYVSRynz3PUSpisDSSaRML5QKX
			hex!["18fc2f1bba77941c731507e33a4b6edfea158f4273acce5b6d598a716243f042"].into(),
			//10 5G18Zda5vBfUsYfaZdp7Q3tPBbVrcbHSPNRvmaGVLtWfBAnX
			hex!["ae371ef015b1b36c6c7e438c07497a822891528e2e6a54a56c3610b7a426987b"].into(),
			//11 5Gmc7XPuiH1oHXzNPEcp1tUh5ZVk4ueEcLSJpErkcR86E1a2
			hex!["d022a8eae4a7cada7e0d49a5c0453eaf0155497649bd079ae3f9db1f72df8c25"]
				.unchecked_into(),
			//12 5EHr9v9RdSfG7ZNitECSugLTCv7UbPvp7Cqyv6wHdcTg9oPK
			hex!["627e96b1eaa650711ec877e9261478e5a9cf36e91868c09bb97c6b9e657b222a"]
				.unchecked_into(),
			//12 5EHr9v9RdSfG7ZNitECSugLTCv7UbPvp7Cqyv6wHdcTg9oPK
			hex!["627e96b1eaa650711ec877e9261478e5a9cf36e91868c09bb97c6b9e657b222a"]
				.unchecked_into(),
			//12 5EHr9v9RdSfG7ZNitECSugLTCv7UbPvp7Cqyv6wHdcTg9oPK
			hex!["627e96b1eaa650711ec877e9261478e5a9cf36e91868c09bb97c6b9e657b222a"]
				.unchecked_into(),
			hex!["627e96b1eaa650711ec877e9261478e5a9cf36e91868c09bb97c6b9e657b222a"]
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		//1 5C8Egtiq4Hw5cA5JmcBetT4wck5ucy7otHu4LbGLwGcH2HXW
		"02b1862bfc7b35281eec139d94903cac1110cedff6b82cffebada8cb7de4815b"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![
		root_key.clone(),
		// Start from 2
		hex!["666b03f17ac057c0dfc1b3ed93f19c119a9884021918e9f7fa8c189a2e621b02"].into(),
		hex!["6ee68b505a25f50f64b4748f8e87ae3c56c4707fcea5cb53527e458a05f8644d"].into(),
		hex!["b0cbb6938aa2aaf89c53e65e0cf1881bbaff26131121b45ea7f2270e4d20d556"].into(),
		hex!["a2fa0f1e725892efec098cc25b17ef9214dbaee50de087bf293e0f9ddcf7b51f"].into(),
		hex!["2ed1562d419c5994d9b6f59b154d9abf49b9c69c5cd0db2d8755e3e58e8bb84e"].into(),
		hex!["7e68092d8db6f74e9ffd3d68d822a01387e6f157e8b14f83e752073dbbaaf242"].into(),
		hex!["fa8d7744f7b2ac8a9c8359e012b548b3f6aba9154861b249fa2fadf405e79924"].into(),
		hex!["18fc2f1bba77941c731507e33a4b6edfea158f4273acce5b6d598a716243f042"].into(),
		hex!["ae371ef015b1b36c6c7e438c07497a822891528e2e6a54a56c3610b7a426987b"].into(),
		hex!["d022a8eae4a7cada7e0d49a5c0453eaf0155497649bd079ae3f9db1f72df8c25"].into(),
		hex!["627e96b1eaa650711ec877e9261478e5a9cf36e91868c09bb97c6b9e657b222a"].into(),
		hex!["0665a589e6da6765902d96dc670749ffe2972c1d39fd7375ecedb83a15edef28"].into(),
		hex!["e2a92e8c51dff02781ed3bd4319250335174e06753c71da485905065dee91c4c"].into(),
		hex!["6c439d092a4c9d88f06397cc53e3f8ebabfc7c2df57b68dddfbe4c3739286673"].into(),
	];


	testnet_genesis(initial_authorities, vec![], root_key, Some(endowed_accounts))
}

pub fn kumandra_testnet_config() -> ChainSpec {
	ChainSpec::from_json_bytes(&include_bytes!("../chains/kumandra-testnet-spec-raw.json")[..]).unwrap()
}

pub fn kumandra_testnet_generate_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Kumandra-Testnet",
		"Kumandra-Testnet",
		ChainType::Live,
		kumandra_testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		Some("TKMD"),
		None,
		Some(
			serde_json::from_str(
				"{\"tokenDecimals\": 12, \"tokenSymbol\": \"TKMD\", \"SS58Prefix\": 11333}",
			)
			.expect("Provided valid json map"),
		),
		Default::default(),
	)
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		development_config_genesis,
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
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
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
		Default::default(),
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		RRSCId,
		ImOnlineId,
		AuthorityDiscoveryId,
		SegmentBookId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
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
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});

	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(&x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 9_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 10;

	GenesisConfig {
		system: SystemConfig { code: wasm_binary_unwrap().to_vec() },
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 65)).collect(),
		},
		indices: IndicesConfig { indices: vec![] },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: 1,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(kumandra_node_runtime::RRSC_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		technical_membership: Default::default(),
		treasury: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		assets: Default::default(),
		transaction_payment: Default::default(),
		evm: Default::default(),
		ethereum: Default::default(),
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
	}
}
