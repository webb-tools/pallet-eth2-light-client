use async_trait::async_trait;
use std::any::Any;

use eth_types::{
	eth2::{ExtendedBeaconBlockHeader, FinalizedHeaderUpdate, SyncCommittee},
	pallet::InitInput,
	BlockHeader,
};
use sp_core::{crypto::AccountId32, sr25519::Pair};
use sp_keyring::AccountKeyring;
use webb::substrate::{
	scale::{Decode, Encode},
	subxt::{
		self,
		dynamic::{DecodedValue, DecodedValueThunk, Value},
		metadata::DecodeWithMetadata,
		tx::{PairSigner, Signer},
		Config, OnlineClient, PolkadotConfig,
	},
};
use webb_proposals::TypedChainId;
use webb_relayer_utils::Error;

use crate::eth_client_pallet_trait::EthClientPalletTrait;

pub async fn setup_api() -> Result<OnlineClient<PolkadotConfig>, Error> {
	let api: OnlineClient<PolkadotConfig> = OnlineClient::<PolkadotConfig>::new().await?;
	Ok(api)
}

async fn init(
	typed_chain_id: TypedChainId,
	finalized_execution_header: BlockHeader,
	finalized_beacon_header: ExtendedBeaconBlockHeader,
	current_sync_committee: SyncCommittee,
	next_sync_committee: SyncCommittee,
	validate_updates: Option<bool>,
	verify_bls_signatures: Option<bool>,
	hashes_gc_threshold: Option<u64>,
	max_submitted_blocks_by_account: Option<u32>,
	trusted_signer: Option<AccountId32>,
) -> Result<(), Error> {
	let api = setup_api().await?;
	let signer = PairSigner::<PolkadotConfig, _>::new(AccountKeyring::Alice.pair());
	// let signer = PairSigner::new(AccountKeyring::Alice.pair());

	let init_input: InitInput<AccountId32> = InitInput {
		finalized_execution_header,
		finalized_beacon_header,
		current_sync_committee,
		next_sync_committee,
		validate_updates: validate_updates.unwrap_or(true),
		verify_bls_signatures: verify_bls_signatures.unwrap_or(true),
		hashes_gc_threshold: hashes_gc_threshold.unwrap_or(100),
		max_submitted_blocks_by_account: max_submitted_blocks_by_account.unwrap_or(10),
		trusted_signer,
	};
	// Create a transaction to submit:
	let tx = subxt::dynamic::tx("Eth2Client", "init", vec![Value::from_bytes(init_input.encode())]);

	// submit the transaction with default params:
	let hash = api.tx().sign_and_submit_default(&tx, &signer).await?;

	Ok(())
}

async fn finalized_beacon_block_slot(typed_chain_id: TypedChainId) -> Result<u64, Error> {
	let api = setup_api().await?;

	let storage_address = subxt::dynamic::storage(
		"Eth2Client",
		"FinalizedBeaconHeader",
		vec![Value::from_bytes(&typed_chain_id.chain_id().to_be_bytes())],
	);
	let finalized_beacon_header_value: DecodedValueThunk =
		api.storage().fetch_or_default(&storage_address, None).await?;

	Ok(0)
}

pub async fn get_last_eth2_slot_on_tangle(typed_chain_id: TypedChainId) -> Result<u64, Error> {
	let api = setup_api().await?;

	let storage_address = subxt::dynamic::storage(
		"Eth2Client",
		"FinalizedHeaderUpdate",
		vec![Value::from_bytes(&typed_chain_id.chain_id().to_be_bytes())],
	);
	let finalized_header_update: DecodedValueThunk =
		api.storage().fetch_or_default(&storage_address, None).await?;

	Ok(0)
}

pub struct EthClientPallet {
	api: OnlineClient<PolkadotConfig>,
	signer: PairSigner<PolkadotConfig, Pair>,
}

impl EthClientPallet {
	pub fn new(api: OnlineClient<PolkadotConfig>) -> Self {
		let signer = PairSigner::new(AccountKeyring::Alice.pair());
		Self { api, signer }
	}

	pub fn get_signer_account_id(&self) -> AccountId32 {
		self.signer.account_id().clone()
	}
}

#[async_trait]
impl EthClientPalletTrait for EthClientPallet {
	async fn get_last_submitted_slot(&self) -> u64 {
		0
	}

	async fn is_known_block(
		&self,
		execution_block_hash: &eth_types::H256,
	) -> Result<bool, Box<dyn std::error::Error>> {
		Ok(false)
	}

	async fn send_light_client_update(
		&mut self,
		light_client_update: eth_types::eth2::LightClientUpdate,
	) -> Result<(), Box<dyn std::error::Error>> {
		Ok(())
	}

	async fn get_finalized_beacon_block_hash(
		&self,
	) -> Result<eth_types::H256, Box<dyn std::error::Error>> {
		Ok(eth_types::H256::from([0u8; 32]))
	}

	async fn get_finalized_beacon_block_slot(&self) -> Result<u64, Box<dyn std::error::Error>> {
		Ok(0)
	}

	async fn send_headers(
		&mut self,
		headers: &[eth_types::BlockHeader],
		end_slot: u64,
	) -> Result<(), Box<dyn std::error::Error>> {
		Ok(())
	}

	async fn get_min_deposit(
		&self,
	) -> Result<crate::eth_client_pallet_trait::Balance, Box<dyn std::error::Error>> {
		Ok(0)
	}

	async fn register_submitter(&self) -> Result<(), Box<dyn std::error::Error>> {
		// Create a transaction to submit:
		let tx =
			subxt::dynamic::tx("Eth2Client", "register_submitter", vec![Value::from_bytes(&[])]);

		// submit the transaction with default params:
		let hash = self.api.tx().sign_and_submit_default(&tx, &self.signer).await?;
		Ok(())
	}

	async fn is_submitter_registered(
		&self,
		account_id: Option<AccountId32>,
	) -> Result<bool, Box<dyn std::error::Error>> {
		Ok(true)
	}

	async fn get_light_client_state(
		&self,
	) -> Result<eth_types::eth2::LightClientState, Box<dyn std::error::Error>> {
		Ok(eth_types::eth2::LightClientState::default())
	}

	async fn get_num_of_submitted_blocks_by_account(
		&self,
	) -> Result<u32, Box<dyn std::error::Error>> {
		Ok(0)
	}

	async fn get_max_submitted_blocks_by_account(&self) -> Result<u32, Box<dyn std::error::Error>> {
		Ok(0)
	}
}
