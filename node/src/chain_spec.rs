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

fn kumandra_main_genesis() -> GenesisConfig {
	#[rustfmt::skip]
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
			// crtfmjMFzKwv2MMimxdK4FCDyVpToU9i9NKUcHoNSxAu5hcoK
			hex!["c8f008a60772513ad739eb81320aeb80a74529fea456094bfbf6203541e0dd42"].into(),
			// crqacqDfzVM9ywsVG2YQaRVPUJEZ3N4S9GKUq7n38gpjHYG9Y
			hex!["404d267d3e7844b1541a940a1d369f534301ad7458b0592622c464f0011bed7a"].into(),
			// crujRrF9RTq1goc9aTDDq37ua2XdqBcJiPBA9eqpncEPfki6R
			hex!["f7f652b0f2de814ec8e88750885997259226c963095bbc2e9754b4ec2c630a49"]
				.unchecked_into(),
			// crqaRonih6bGs675axWkNdH3DMF5XWzvQYf2DPTyWSfzPPSBc
			hex!["402809bcd4b6370e02764319bfbf37c09abed526ae5b5e2bb363c20876404358"]
				.unchecked_into(),
			// crqaRonih6bGs675axWkNdH3DMF5XWzvQYf2DPTyWSfzPPSBc
			hex!["402809bcd4b6370e02764319bfbf37c09abed526ae5b5e2bb363c20876404358"]
				.unchecked_into(),
			// crqaRonih6bGs675axWkNdH3DMF5XWzvQYf2DPTyWSfzPPSBc
			hex!["402809bcd4b6370e02764319bfbf37c09abed526ae5b5e2bb363c20876404358"]
				.unchecked_into(),
			hex!["402809bcd4b6370e02764319bfbf37c09abed526ae5b5e2bb363c20876404358"]
				.unchecked_into(),
		),
		(
			// crsaFRsVsJmW1te3k2mdER5ePz6hqumzHgMuXXcDVLnsvUj1F
			hex!["987d91e1daa7030f5fd23ec00fb661970b3dc158bdf3f65ee9c3e94481901a1a"].into(),
			// crpiqLT5iQoMnFK2U6JrGCm34ZTAGEZHT3oPcf4GZtVDrEdwA
			hex!["1a54f8e8a08aac9d72487a6eca0d4dbf43b0236a19a2115fe3151064e626ab45"].into(),
			// cruFoW2bHbkNYMziZ4T1SpqbtXX2XePyU1WjrFNgw4jZF19C3
			hex!["e2e4564174a8e1d27e2db5d405492b1fbd6da47e3c1cc522734397a6565f7698"]
				.unchecked_into(),
			// crqDjWp8yJgVm7XEvuKBCkEHLw2uK58xFrycS4HtXkydAp19K
			hex!["305f7e211cda884c763579939963c1ebff0ba0f71e0b0fd6776cb00bd09fcd5f"]
				.unchecked_into(),
			// crqDjWp8yJgVm7XEvuKBCkEHLw2uK58xFrycS4HtXkydAp19K
			hex!["305f7e211cda884c763579939963c1ebff0ba0f71e0b0fd6776cb00bd09fcd5f"]
				.unchecked_into(),
			// crqDjWp8yJgVm7XEvuKBCkEHLw2uK58xFrycS4HtXkydAp19K
			hex!["305f7e211cda884c763579939963c1ebff0ba0f71e0b0fd6776cb00bd09fcd5f"]
				.unchecked_into(),
			hex!["305f7e211cda884c763579939963c1ebff0ba0f71e0b0fd6776cb00bd09fcd5f"]
				.unchecked_into(),
		),
		(
			// crtkaKE4cFVGFJCYWx3y4RZzyJg251Gg3VA7VfBR41b9yUtyk
			hex!["cc99d4d373886767ad13372c3dfb5357fb49fcbc24179d267095839f6d6f2c29"].into(),
			// cXj4Z6VmYHkD64xEadQCNgFUDaV4F4ibeupuU4ELJa8uWH7mw
			hex!["6caea2cfaf05154125ede6bec9cd05c8b5b5262daf96eb32556170258b1fa64f"].into(),
			// cXjQgQVu9Zuitzkb8oiTYTKt7QUm9jupKbzqVNVwPELwz26rT
			hex!["84aad7c1fee1827ee747a9c7cf927b91860b22b438413ef5b8da7c7e8cf5521c"]
				.unchecked_into(),
			// crqwfyyMoqZdJB4iBRrdT8pmL59qFfUW36GiK4o7j9Eg7Wzek
			hex!["505be9287c32386f4d6bd390e9a9d5715ace277c344312d18dd4c0e75ab62526"]
				.unchecked_into(),
			// crqwfyyMoqZdJB4iBRrdT8pmL59qFfUW36GiK4o7j9Eg7Wzek
			hex!["505be9287c32386f4d6bd390e9a9d5715ace277c344312d18dd4c0e75ab62526"]
				.unchecked_into(),
			// crqwfyyMoqZdJB4iBRrdT8pmL59qFfUW36GiK4o7j9Eg7Wzek
			hex!["505be9287c32386f4d6bd390e9a9d5715ace277c344312d18dd4c0e75ab62526"]
				.unchecked_into(),
			//crqwfyyMoqZdJB4iBRrdT8pmL59qFfUW36GiK4o7j9Eg7Wzek
			hex!["505be9287c32386f4d6bd390e9a9d5715ace277c344312d18dd4c0e75ab62526"]
				.unchecked_into(),
		),
		(
			// crraNo4MSxSaHx6DRUJw2AsY6DnR39gV1jX2Njhz62NZ1CuMJ
			hex!["6c5a2154773c0ef38c667dea493ce9cf2a1ef5e1404375768f91fa76880e6974"].into(),
			// crtfabWTFvtU2E9W87Gsiit9k65KfsixTxTPkjLQJxqcLrt9n
			hex!["c8ca8ca33e6d2b984974d0a3fe10d7ce27dcd6f1420c62285f0d95fca8b7912a"].into(),
			// crsAvYoDYgeaAsZiuuJ3cNJ1YbUcUFjaTiGyY84qxK9FiX4TE
			hex!["86b35e2b06c72b9ea3881b39690e3949b81153e78d27002d7b9233ee3b44e215"]
				.unchecked_into(),
			// crtCNRW1XGaeaN5Y2Hojgyv5r2caHGZtjYTgvcauWAUyiwSwc
			hex!["b409f26f072212c61e38cc7dd7fcc95f65965661c5487d05eb766df61385547e"]
				.unchecked_into(),
			// crtCNRW1XGaeaN5Y2Hojgyv5r2caHGZtjYTgvcauWAUyiwSwc
			hex!["b409f26f072212c61e38cc7dd7fcc95f65965661c5487d05eb766df61385547e"]
				.unchecked_into(),
			// crtCNRW1XGaeaN5Y2Hojgyv5r2caHGZtjYTgvcauWAUyiwSwc
			hex!["b409f26f072212c61e38cc7dd7fcc95f65965661c5487d05eb766df61385547e"]
				.unchecked_into(),
			// crtCNRW1XGaeaN5Y2Hojgyv5r2caHGZtjYTgvcauWAUyiwSwc
			hex!["b409f26f072212c61e38cc7dd7fcc95f65965661c5487d05eb766df61385547e"]
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// cXffK7BmstE5rXcK8pxKLufkffp9iASMntxUm6ctpR6xS3icV
		"404d267d3e7844b1541a940a1d369f534301ad7458b0592622c464f0011bed7a"
	]
		.into();

	let endowed_accounts: Vec<AccountId> = vec![
		root_key.clone(),
		hex!["c8f008a60772513ad739eb81320aeb80a74529fea456094bfbf6203541e0dd42"].into(),
		hex!["f7f652b0f2de814ec8e88750885997259226c963095bbc2e9754b4ec2c630a49"].into(),
		hex!["402809bcd4b6370e02764319bfbf37c09abed526ae5b5e2bb363c20876404358"].into(),
		hex!["987d91e1daa7030f5fd23ec00fb661970b3dc158bdf3f65ee9c3e94481901a1a"].into(),
		hex!["1a54f8e8a08aac9d72487a6eca0d4dbf43b0236a19a2115fe3151064e626ab45"].into(),
		hex!["e2e4564174a8e1d27e2db5d405492b1fbd6da47e3c1cc522734397a6565f7698"].into(),
		hex!["305f7e211cda884c763579939963c1ebff0ba0f71e0b0fd6776cb00bd09fcd5f"].into(),
		hex!["cc99d4d373886767ad13372c3dfb5357fb49fcbc24179d267095839f6d6f2c29"].into(),
		hex!["6caea2cfaf05154125ede6bec9cd05c8b5b5262daf96eb32556170258b1fa64f"].into(),
		hex!["84aad7c1fee1827ee747a9c7cf927b91860b22b438413ef5b8da7c7e8cf5521c"].into(),
		hex!["505be9287c32386f4d6bd390e9a9d5715ace277c344312d18dd4c0e75ab62526"].into(),
		hex!["1a48bc9d179ffd4ea4131c56a4bcf1246023c6d1d5a2ad28c6564422ae65d24f"].into(),
		hex!["4c023836f595515248e598d91cec570d06b0d6ae590a1716543b96ffa9859a72"].into(),
		hex!["7031884355b884f532e04f02a69158a3baaf97b40bb95035ab7b869709fa261d"].into(),
		hex!["6c5a2154773c0ef38c667dea493ce9cf2a1ef5e1404375768f91fa76880e6974"].into(),
		hex!["c8ca8ca33e6d2b984974d0a3fe10d7ce27dcd6f1420c62285f0d95fca8b7912a"].into(),
		hex!["86b35e2b06c72b9ea3881b39690e3949b81153e78d27002d7b9233ee3b44e215"].into(),
		hex!["b409f26f072212c61e38cc7dd7fcc95f65965661c5487d05eb766df61385547e"].into(),
	];

	testnet_genesis(initial_authorities, vec![], root_key, Some(endowed_accounts))
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
			// crtfmjMFzKwv2MMimxdK4FCDyVpToU9i9NKUcHoNSxAu5hcoK
			hex!["c8f008a60772513ad739eb81320aeb80a74529fea456094bfbf6203541e0dd42"].into(),
			// crqacqDfzVM9ywsVG2YQaRVPUJEZ3N4S9GKUq7n38gpjHYG9Y
			hex!["404d267d3e7844b1541a940a1d369f534301ad7458b0592622c464f0011bed7a"].into(),
			// crujRrF9RTq1goc9aTDDq37ua2XdqBcJiPBA9eqpncEPfki6R
			hex!["f7f652b0f2de814ec8e88750885997259226c963095bbc2e9754b4ec2c630a49"]
				.unchecked_into(),
			// crqaRonih6bGs675axWkNdH3DMF5XWzvQYf2DPTyWSfzPPSBc
			hex!["402809bcd4b6370e02764319bfbf37c09abed526ae5b5e2bb363c20876404358"]
				.unchecked_into(),
			// crqaRonih6bGs675axWkNdH3DMF5XWzvQYf2DPTyWSfzPPSBc
			hex!["402809bcd4b6370e02764319bfbf37c09abed526ae5b5e2bb363c20876404358"]
				.unchecked_into(),
			// crqaRonih6bGs675axWkNdH3DMF5XWzvQYf2DPTyWSfzPPSBc
			hex!["402809bcd4b6370e02764319bfbf37c09abed526ae5b5e2bb363c20876404358"]
				.unchecked_into(),
			hex!["402809bcd4b6370e02764319bfbf37c09abed526ae5b5e2bb363c20876404358"]
				.unchecked_into(),
		),
		(
			// crsaFRsVsJmW1te3k2mdER5ePz6hqumzHgMuXXcDVLnsvUj1F
			hex!["987d91e1daa7030f5fd23ec00fb661970b3dc158bdf3f65ee9c3e94481901a1a"].into(),
			// crpiqLT5iQoMnFK2U6JrGCm34ZTAGEZHT3oPcf4GZtVDrEdwA
			hex!["1a54f8e8a08aac9d72487a6eca0d4dbf43b0236a19a2115fe3151064e626ab45"].into(),
			// cruFoW2bHbkNYMziZ4T1SpqbtXX2XePyU1WjrFNgw4jZF19C3
			hex!["e2e4564174a8e1d27e2db5d405492b1fbd6da47e3c1cc522734397a6565f7698"]
				.unchecked_into(),
			// crqDjWp8yJgVm7XEvuKBCkEHLw2uK58xFrycS4HtXkydAp19K
			hex!["305f7e211cda884c763579939963c1ebff0ba0f71e0b0fd6776cb00bd09fcd5f"]
				.unchecked_into(),
			// crqDjWp8yJgVm7XEvuKBCkEHLw2uK58xFrycS4HtXkydAp19K
			hex!["305f7e211cda884c763579939963c1ebff0ba0f71e0b0fd6776cb00bd09fcd5f"]
				.unchecked_into(),
			// crqDjWp8yJgVm7XEvuKBCkEHLw2uK58xFrycS4HtXkydAp19K
			hex!["305f7e211cda884c763579939963c1ebff0ba0f71e0b0fd6776cb00bd09fcd5f"]
				.unchecked_into(),
			hex!["305f7e211cda884c763579939963c1ebff0ba0f71e0b0fd6776cb00bd09fcd5f"]
				.unchecked_into(),
		),
		(
			// crtkaKE4cFVGFJCYWx3y4RZzyJg251Gg3VA7VfBR41b9yUtyk
			hex!["cc99d4d373886767ad13372c3dfb5357fb49fcbc24179d267095839f6d6f2c29"].into(),
			// cXj4Z6VmYHkD64xEadQCNgFUDaV4F4ibeupuU4ELJa8uWH7mw
			hex!["6caea2cfaf05154125ede6bec9cd05c8b5b5262daf96eb32556170258b1fa64f"].into(),
			// cXjQgQVu9Zuitzkb8oiTYTKt7QUm9jupKbzqVNVwPELwz26rT
			hex!["84aad7c1fee1827ee747a9c7cf927b91860b22b438413ef5b8da7c7e8cf5521c"]
				.unchecked_into(),
			// crqwfyyMoqZdJB4iBRrdT8pmL59qFfUW36GiK4o7j9Eg7Wzek
			hex!["505be9287c32386f4d6bd390e9a9d5715ace277c344312d18dd4c0e75ab62526"]
				.unchecked_into(),
			// crqwfyyMoqZdJB4iBRrdT8pmL59qFfUW36GiK4o7j9Eg7Wzek
			hex!["505be9287c32386f4d6bd390e9a9d5715ace277c344312d18dd4c0e75ab62526"]
				.unchecked_into(),
			// crqwfyyMoqZdJB4iBRrdT8pmL59qFfUW36GiK4o7j9Eg7Wzek
			hex!["505be9287c32386f4d6bd390e9a9d5715ace277c344312d18dd4c0e75ab62526"]
				.unchecked_into(),
			//crqwfyyMoqZdJB4iBRrdT8pmL59qFfUW36GiK4o7j9Eg7Wzek
			hex!["505be9287c32386f4d6bd390e9a9d5715ace277c344312d18dd4c0e75ab62526"]
				.unchecked_into(),
		),
		(
			// crraNo4MSxSaHx6DRUJw2AsY6DnR39gV1jX2Njhz62NZ1CuMJ
			hex!["6c5a2154773c0ef38c667dea493ce9cf2a1ef5e1404375768f91fa76880e6974"].into(),
			// crtfabWTFvtU2E9W87Gsiit9k65KfsixTxTPkjLQJxqcLrt9n
			hex!["c8ca8ca33e6d2b984974d0a3fe10d7ce27dcd6f1420c62285f0d95fca8b7912a"].into(),
			// crsAvYoDYgeaAsZiuuJ3cNJ1YbUcUFjaTiGyY84qxK9FiX4TE
			hex!["86b35e2b06c72b9ea3881b39690e3949b81153e78d27002d7b9233ee3b44e215"]
				.unchecked_into(),
			// crtCNRW1XGaeaN5Y2Hojgyv5r2caHGZtjYTgvcauWAUyiwSwc
			hex!["b409f26f072212c61e38cc7dd7fcc95f65965661c5487d05eb766df61385547e"]
				.unchecked_into(),
			// crtCNRW1XGaeaN5Y2Hojgyv5r2caHGZtjYTgvcauWAUyiwSwc
			hex!["b409f26f072212c61e38cc7dd7fcc95f65965661c5487d05eb766df61385547e"]
				.unchecked_into(),
			// crtCNRW1XGaeaN5Y2Hojgyv5r2caHGZtjYTgvcauWAUyiwSwc
			hex!["b409f26f072212c61e38cc7dd7fcc95f65965661c5487d05eb766df61385547e"]
				.unchecked_into(),
			// crtCNRW1XGaeaN5Y2Hojgyv5r2caHGZtjYTgvcauWAUyiwSwc
			hex!["b409f26f072212c61e38cc7dd7fcc95f65965661c5487d05eb766df61385547e"]
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// cXffK7BmstE5rXcK8pxKLufkffp9iASMntxUm6ctpR6xS3icV
		"404d267d3e7844b1541a940a1d369f534301ad7458b0592622c464f0011bed7a"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![
		root_key.clone(),
		hex!["c8f008a60772513ad739eb81320aeb80a74529fea456094bfbf6203541e0dd42"].into(),
		hex!["f7f652b0f2de814ec8e88750885997259226c963095bbc2e9754b4ec2c630a49"].into(),
		hex!["402809bcd4b6370e02764319bfbf37c09abed526ae5b5e2bb363c20876404358"].into(),
		hex!["987d91e1daa7030f5fd23ec00fb661970b3dc158bdf3f65ee9c3e94481901a1a"].into(),
		hex!["1a54f8e8a08aac9d72487a6eca0d4dbf43b0236a19a2115fe3151064e626ab45"].into(),
		hex!["e2e4564174a8e1d27e2db5d405492b1fbd6da47e3c1cc522734397a6565f7698"].into(),
		hex!["305f7e211cda884c763579939963c1ebff0ba0f71e0b0fd6776cb00bd09fcd5f"].into(),
		hex!["cc99d4d373886767ad13372c3dfb5357fb49fcbc24179d267095839f6d6f2c29"].into(),
		hex!["6caea2cfaf05154125ede6bec9cd05c8b5b5262daf96eb32556170258b1fa64f"].into(),
		hex!["84aad7c1fee1827ee747a9c7cf927b91860b22b438413ef5b8da7c7e8cf5521c"].into(),
		hex!["505be9287c32386f4d6bd390e9a9d5715ace277c344312d18dd4c0e75ab62526"].into(),
		hex!["1a48bc9d179ffd4ea4131c56a4bcf1246023c6d1d5a2ad28c6564422ae65d24f"].into(),
		hex!["4c023836f595515248e598d91cec570d06b0d6ae590a1716543b96ffa9859a72"].into(),
		hex!["7031884355b884f532e04f02a69158a3baaf97b40bb95035ab7b869709fa261d"].into(),
	];

	testnet_genesis(initial_authorities, vec![], root_key, Some(endowed_accounts))
}

pub fn kumandra_testnet_config() -> ChainSpec {
	ChainSpec::from_json_bytes(&include_bytes!("../chains/kumandra-testnet-spec-raw.json")[..]).unwrap()
}

pub fn kumandra_develop_config() -> ChainSpec {
	ChainSpec::from_json_bytes(&include_bytes!("../chains/kumandra-develop-spec-raw.json")[..]).unwrap()
}

pub fn kumandra_testnet_generate_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"kumandra-testnet",
		"kumandra-testnet",
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

// TODO Mainnet Chainspec

pub fn kumandra_main() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"kumandra-testnet",
		"kumandra-testnet",
		ChainType::Live,
		kumandra_main_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		Some("KMD"),
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

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
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
