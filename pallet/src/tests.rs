use super::consensus::*;
use crate::{
	mock::{Eth2Client, Origin},
	test_utils::*,
	types::InitInput,
};
use bitvec::{bitarr, order::Lsb0};
use eth_types::{eth2::LightClientUpdate, BlockHeader, H256, U256};
use frame_support::{assert_err, assert_ok};
use hex::FromHex;
use sp_runtime::AccountId32;
use tree_hash::TreeHash;
use webb_proposals::TypedChainId;

const MAINNET_CHAIN: TypedChainId = TypedChainId::Evm(1);
const KILN_CHAIN: TypedChainId = TypedChainId::Evm(1337802);
const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

pub fn submit_and_check_execution_headers(
	origin: Origin,
	typed_chain_id: TypedChainId,
	headers: Vec<&BlockHeader>,
) {
	for header in headers {
		assert_ok!(Eth2Client::submit_execution_header(
			origin.clone(),
			typed_chain_id,
			header.clone()
		));
		assert!(Eth2Client::is_known_execution_header(typed_chain_id, header.calculate_hash()));
		assert!(Eth2Client::block_hash_safe(typed_chain_id, header.number).is_none());
	}
}

pub fn get_test_context(
	init_options: Option<InitOptions<AccountId32>>,
) -> (&'static Vec<BlockHeader>, &'static Vec<LightClientUpdate>, InitInput<AccountId32>) {
	let (headers, updates, init_input) = get_test_data(init_options);
	assert_ok!(Eth2Client::init(
		Origin::signed(ALICE.clone()),
		KILN_CHAIN,
		Box::new(init_input.clone())
	));
	(headers, updates, init_input)
}

mod kiln_tests {
	use super::*;
	use crate::{
		mock::{new_test_ext, Eth2Client, Test},
		test_utils::read_beacon_header,
		Error, Paused,
	};

	#[test]
	pub fn test_header_root() {
		let header = read_beacon_header(format!("./src/data/kiln/beacon_header_{}.json", 5000));
		assert_eq!(
			H256(header.tree_hash_root()),
			Vec::from_hex("c613fbf1a8e95c2aa0f76a5d226ee1dc057cce18b235803f50e7a1bde050d290")
				.unwrap()
				.into()
		);

		let header =
			read_beacon_header(format!("./src/data/mainnet/beacon_header_{}.json", 4100000));
		assert_eq!(
			H256(header.tree_hash_root()),
			Vec::from_hex("342ca1455e976f300cc96a209106bed2cbdf87243167fab61edc6e2250a0be6c")
				.unwrap()
				.into()
		);
	}

	#[test]
	pub fn test_submit_update_two_periods() {
		new_test_ext().execute_with(|| {
			let (headers, updates, _init_input) = get_test_context(None);
			assert_ok!(Eth2Client::register_submitter(Origin::signed(ALICE), KILN_CHAIN));
			// After submitting the execution header, it should be present in the execution headers
			// list but absent in canonical chain blocks (not-finalized)
			submit_and_check_execution_headers(
				Origin::signed(ALICE),
				KILN_CHAIN,
				headers.iter().skip(1).collect(),
			);

			assert_ok!(Eth2Client::submit_beacon_chain_light_client_update(
				Origin::signed(ALICE),
				KILN_CHAIN,
				updates[1].clone()
			));

			// After Beacon Chain `LightClientUpdate` is submitted,
			// all execution headers having a height lower than the update's height,
			// should be removed from the execution headers list. Meantime, all these
			// removed execution headers should become a part of the canonical chain blocks
			// (finalized)
			for header in headers.iter().skip(1) {
				let header_hash = header.calculate_hash();
				assert!(!Eth2Client::is_known_execution_header(KILN_CHAIN, header_hash));
				assert!(
					Eth2Client::block_hash_safe(KILN_CHAIN, header.number).unwrap_or_default() ==
						header_hash,
					"Execution block hash is not finalized: {:?}",
					header_hash
				);
			}

			assert_eq!(Eth2Client::last_block_number(KILN_CHAIN), headers.last().unwrap().number);
			assert!(!Eth2Client::is_known_execution_header(
				KILN_CHAIN,
				Eth2Client::finalized_beacon_block_header(KILN_CHAIN)
					.unwrap()
					.execution_block_hash
			));

			assert_ok!(Eth2Client::unregister_submitter(Origin::signed(ALICE), KILN_CHAIN));
		})
	}

	#[test]
	pub fn test_submit_execution_block_from_fork_chain() {
		new_test_ext().execute_with(|| {
			let (headers, updates, _init_input) = get_test_context(None);
			assert_ok!(Eth2Client::register_submitter(Origin::signed(ALICE), KILN_CHAIN));
			// After submitting the execution header, it should be present in the execution headers
			// list but absent in canonical chain blocks (not-finalized)
			submit_and_check_execution_headers(
				Origin::signed(ALICE),
				KILN_CHAIN,
				headers.iter().skip(1).collect(),
			);
			// Submit execution header with different hash
			let mut fork_header = headers[5].clone();
			// Difficulty is modified just in order to get a different header hash. Any other field
			// would be suitable too
			fork_header.difficulty = U256::from(ethereum_types::U256::from(99));
			assert_ok!(Eth2Client::submit_execution_header(
				Origin::signed(ALICE),
				KILN_CHAIN,
				fork_header.clone()
			));
			assert_ok!(Eth2Client::submit_beacon_chain_light_client_update(
				Origin::signed(ALICE),
				KILN_CHAIN,
				updates[1].clone()
			));

			for header in headers.iter().skip(1) {
				let header_hash = header.calculate_hash();
				assert!(!Eth2Client::is_known_execution_header(KILN_CHAIN, header_hash));
				assert!(
					Eth2Client::block_hash_safe(KILN_CHAIN, header.number).unwrap_or_default() ==
						header_hash,
					"Execution block hash is not finalized: {:?}",
					header_hash
				);
			}

			// Check that forked execution header was not finalized
			assert!(Eth2Client::is_known_execution_header(
				KILN_CHAIN,
				fork_header.calculate_hash()
			));
			assert!(
				Eth2Client::block_hash_safe(KILN_CHAIN, fork_header.number).unwrap_or_default()
				!= fork_header.calculate_hash(),
				"The fork's execution block header {:?} is expected not to be finalized, but it is finalized",
				fork_header.calculate_hash()
			);

			assert_eq!(Eth2Client::last_block_number(KILN_CHAIN), headers.last().unwrap().number);
		});
	}

	#[test]
	pub fn test_gc_headers() {
		new_test_ext().execute_with(|| {
			let (headers, updates, _init_input) = get_test_context(Some(InitOptions {
				validate_updates: true,
				verify_bls_signatures: true,
				hashes_gc_threshold: 500,
				max_submitted_blocks_by_account: 7000,
				trusted_signer: None,
			}));
			assert_ok!(Eth2Client::register_submitter(Origin::signed(ALICE), KILN_CHAIN));
			// After submitting the execution header, it should be present in the execution headers
			// list but absent in canonical chain blocks (not-finalized)
			submit_and_check_execution_headers(
				Origin::signed(ALICE),
				KILN_CHAIN,
				headers.iter().skip(1).collect(),
			);

			assert_ok!(Eth2Client::submit_beacon_chain_light_client_update(
				Origin::signed(ALICE),
				KILN_CHAIN,
				updates[1].clone()
			));

			// Last 500 execution headers are finalized
			for header in headers.iter().skip(1).rev().take(500) {
				assert!(!Eth2Client::is_known_execution_header(
					KILN_CHAIN,
					header.calculate_hash()
				));
				assert!(
					Eth2Client::block_hash_safe(KILN_CHAIN, header.number).unwrap_or_default() ==
						header.calculate_hash(),
					"Execution block hash is not finalized: {:?}",
					header.calculate_hash()
				);
			}

			assert_eq!(Eth2Client::last_block_number(KILN_CHAIN,), headers.last().unwrap().number);

			// Headers older than last 500 hundred headers are both removed and are not present in
			// execution header list
			for header in headers.iter().skip(1).rev().skip(500) {
				assert!(!Eth2Client::is_known_execution_header(
					KILN_CHAIN,
					header.calculate_hash()
				));
				assert!(
					Eth2Client::block_hash_safe(KILN_CHAIN, header.number).is_none(),
					"Execution block hash was not removed: {:?}",
					header.calculate_hash()
				);
			}
		})
	}

	#[test]
	pub fn test_panic_on_exhausted_submit_limit() {
		new_test_ext().execute_with(|| {
			let (headers, _updates, _init_input) = get_test_context(Some(InitOptions {
				validate_updates: true,
				verify_bls_signatures: true,
				hashes_gc_threshold: 7100,
				max_submitted_blocks_by_account: 100,
				trusted_signer: None,
			}));
			assert_ok!(Eth2Client::register_submitter(Origin::signed(ALICE), KILN_CHAIN));
			// After submitting the execution header, it should be present in the execution headers
			// list but absent in canonical chain blocks (not-finalized)
			submit_and_check_execution_headers(
				Origin::signed(ALICE),
				KILN_CHAIN,
				headers.iter().skip(1).take(100).collect(),
			);
			assert_err!(
				Eth2Client::submit_execution_header(
					Origin::signed(ALICE),
					KILN_CHAIN,
					headers[101].clone()
				),
				Error::<Test>::SubmitterExhaustedLimit
			);
		});
	}

	#[test]
	pub fn test_max_submit_blocks_by_account_limit() {
		new_test_ext().execute_with(|| {
			let (headers, _updates, _init_input) = get_test_context(Some(InitOptions {
				validate_updates: true,
				verify_bls_signatures: true,
				hashes_gc_threshold: 7100,
				max_submitted_blocks_by_account: 100,
				trusted_signer: None,
			}));
			assert_ok!(Eth2Client::register_submitter(Origin::signed(ALICE), KILN_CHAIN));
			submit_and_check_execution_headers(
				Origin::signed(ALICE),
				KILN_CHAIN,
				headers.iter().skip(1).take(100).collect(),
			);
		});
	}

	#[test]
	pub fn test_trusted_signer() {
		new_test_ext().execute_with(|| {
			let (_headers, updates, _init_input) = get_test_context(Some(InitOptions {
				validate_updates: true,
				verify_bls_signatures: true,
				hashes_gc_threshold: 7100,
				max_submitted_blocks_by_account: 100,
				trusted_signer: Some(AccountId32::from([2u8; 32])),
			}));
			assert_err!(
				Eth2Client::submit_beacon_chain_light_client_update(
					Origin::signed(ALICE),
					KILN_CHAIN,
					updates[1].clone()
				),
				Error::<Test>::NotTrustedSigner,
			);
		});
	}

	#[test]
	pub fn test_panic_on_invalid_finality_proof() {
		new_test_ext().execute_with(|| {
			let (_headers, updates, _init_input) = get_test_context(None);
			let mut update = updates[1].clone();
			update.finality_update.finality_branch[5] = H256::from(
				hex::decode("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
					.unwrap(),
			);
			assert_err!(
				Eth2Client::submit_beacon_chain_light_client_update(
					Origin::signed(ALICE),
					KILN_CHAIN,
					update
				),
				Error::<Test>::InvalidFinalityProof,
			);
		});
	}

	#[test]
	pub fn test_panic_on_empty_finality_proof() {
		new_test_ext().execute_with(|| {
			let (_headers, updates, _init_input) = get_test_context(None);
			let mut update = updates[1].clone();
			update.finality_update.finality_branch = vec![];
			assert_err!(
				Eth2Client::submit_beacon_chain_light_client_update(
					Origin::signed(ALICE),
					KILN_CHAIN,
					update
				),
				Error::<Test>::InvalidFinalityProof,
			);
		});
	}

	#[test]
	pub fn test_panic_on_invalid_execution_block_proof() {
		new_test_ext().execute_with(|| {
			let (_headers, updates, _init_input) = get_test_context(None);
			let mut update = updates[1].clone();
			update.finality_update.header_update.execution_hash_branch[5] = H256::from(
				hex::decode("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
					.unwrap(),
			);
			assert_err!(
				Eth2Client::submit_beacon_chain_light_client_update(
					Origin::signed(ALICE),
					KILN_CHAIN,
					update
				),
				Error::<Test>::InvalidExecutionBlockHashProof
			);
		});
	}

	#[test]
	pub fn test_panic_on_empty_execution_block_proof() {
		new_test_ext().execute_with(|| {
			let (_headers, updates, _init_input) = get_test_context(None);
			let mut update = updates[1].clone();
			update.finality_update.header_update.execution_hash_branch = vec![];
			assert_err!(
				Eth2Client::submit_beacon_chain_light_client_update(
					Origin::signed(ALICE),
					KILN_CHAIN,
					update
				),
				Error::<Test>::InvalidExecutionBlockHashProof
			);
		});
	}

	#[test]
	pub fn test_panic_on_skip_update_period() {
		new_test_ext().execute_with(|| {
			let (_headers, updates, _init_input) = get_test_context(None);
			let mut update = updates[1].clone();
			update.finality_update.header_update.beacon_header.slot =
				update.signature_slot + EPOCHS_PER_SYNC_COMMITTEE_PERIOD * SLOTS_PER_EPOCH * 10;
			assert_err!(
				Eth2Client::submit_beacon_chain_light_client_update(
					Origin::signed(ALICE),
					KILN_CHAIN,
					update
				),
				Error::<Test>::InvalidUpdatePeriod
			);
		});
	}

	#[test]
	pub fn test_panic_on_submit_update_with_missing_execution_blocks() {
		new_test_ext().execute_with(|| {
			let (headers, updates, _init_input) = get_test_context(None);
			assert_ok!(Eth2Client::register_submitter(Origin::signed(ALICE), KILN_CHAIN));
			submit_and_check_execution_headers(
				Origin::signed(ALICE),
				KILN_CHAIN,
				headers.iter().skip(1).take(5).collect(),
			);
			assert_err!(
				Eth2Client::submit_beacon_chain_light_client_update(
					Origin::signed(ALICE),
					KILN_CHAIN,
					updates[1].clone()
				),
				Error::<Test>::FinalizedExecutionHeaderNotPresent
			);
		});
	}

	#[test]
	pub fn test_panic_on_submit_same_execution_blocks() {
		new_test_ext().execute_with(|| {
			let (headers, _updates, _init_input) = get_test_context(None);
			assert_ok!(Eth2Client::register_submitter(Origin::signed(ALICE), KILN_CHAIN));
			assert_ok!(Eth2Client::submit_execution_header(
				Origin::signed(ALICE),
				KILN_CHAIN,
				headers[1].clone()
			));
			assert_err!(
				Eth2Client::submit_execution_header(
					Origin::signed(ALICE),
					KILN_CHAIN,
					headers[1].clone()
				),
				Error::<Test>::BlockAlreadySubmitted
			);
		});
	}

	#[test]
	pub fn test_panic_on_submit_execution_block_after_submitter_unregistered() {
		new_test_ext().execute_with(|| {
			let (headers, _updates, _init_input) = get_test_context(None);
			assert_ok!(Eth2Client::register_submitter(Origin::signed(ALICE), KILN_CHAIN));
			assert_ok!(Eth2Client::unregister_submitter(Origin::signed(ALICE), KILN_CHAIN));
			assert_err!(
				Eth2Client::submit_execution_header(
					Origin::signed(ALICE),
					KILN_CHAIN,
					headers[1].clone()
				),
				Error::<Test>::SubmitterNotRegistered,
			);
		});
	}

	#[test]
	pub fn test_panic_on_submit_update_paused() {
		new_test_ext().execute_with(|| {
			let (_headers, updates, _init_input) = get_test_context(None);
			Paused::<Test>::insert(KILN_CHAIN, true);
			assert_err!(
				Eth2Client::submit_beacon_chain_light_client_update(
					Origin::signed(ALICE),
					KILN_CHAIN,
					updates[1].clone()
				),
				Error::<Test>::LightClientUpdateNotAllowed
			);
		});
	}

	#[test]
	pub fn test_panic_on_submit_outdated_update() {
		new_test_ext().execute_with(|| {
			let (_headers, updates, _init_input) = get_test_context(None);
			assert_err!(
				Eth2Client::submit_beacon_chain_light_client_update(
					Origin::signed(ALICE),
					KILN_CHAIN,
					updates[0].clone()
				),
				Error::<Test>::ActiveHeaderSlotNumberLessThanFinalizedSlot,
			);
		});
	}

	#[test]
	pub fn test_panic_on_submit_blocks_with_unknown_parent() {
		new_test_ext().execute_with(|| {
			let (headers, _updates, _init_input) = get_test_context(None);
			assert_eq!(Eth2Client::last_block_number(KILN_CHAIN), headers[0].number);
			assert_ok!(Eth2Client::register_submitter(Origin::signed(ALICE), KILN_CHAIN));
			assert_ok!(Eth2Client::submit_execution_header(
				Origin::signed(ALICE),
				KILN_CHAIN,
				headers[1].clone()
			));
			// Skip 2th block
			assert_err!(
				Eth2Client::submit_execution_header(
					Origin::signed(ALICE),
					KILN_CHAIN,
					headers[3].clone()
				),
				Error::<Test>::UnknownParentHeader
			);
		});
	}

	#[test]
	pub fn test_panic_on_unregister_submitter() {
		new_test_ext().execute_with(|| {
			let (headers, _updates, _init_input) = get_test_context(None);
			assert_eq!(Eth2Client::last_block_number(KILN_CHAIN), headers[0].number);
			assert_ok!(Eth2Client::register_submitter(Origin::signed(ALICE), KILN_CHAIN));
			submit_and_check_execution_headers(
				Origin::signed(ALICE),
				KILN_CHAIN,
				headers.iter().skip(1).take(5).collect(),
			);

			assert_err!(
				Eth2Client::unregister_submitter(Origin::signed(ALICE), KILN_CHAIN),
				Error::<Test>::SubmitterHasUsedStorage,
			);
		});
	}

	#[test]
	pub fn test_panic_on_skipping_register_submitter() {
		new_test_ext().execute_with(|| {
			let (headers, _updates, _init_input) = get_test_context(None);
			assert_eq!(Eth2Client::last_block_number(KILN_CHAIN), headers[0].number);
			assert_err!(
				Eth2Client::submit_execution_header(
					Origin::signed(ALICE),
					KILN_CHAIN,
					headers[1].clone()
				),
				Error::<Test>::SubmitterNotRegistered,
			);
		});
	}

	#[test]
	pub fn test_panic_on_sync_committee_bits_is_less_than_threshold() {
		new_test_ext().execute_with(|| {
			let (_headers, updates, _init_input) = get_test_context(None);
			let mut update = updates[1].clone();

			let mut sync_committee_bits = bitarr![u8, Lsb0; 0; 512];

			// The number of participants should satisfy the inequality:
			// num_of_participants * 3 >= sync_committee_bits_size * 2
			// If the sync_committee_bits_size = 512, then
			// the minimum allowed value of num_of_participants is 342.

			// Fill the sync_committee_bits with 341 participants to trigger panic
			let num_of_participants = (((512.0 * 2.0 / 3.0) as f32).ceil() - 1.0) as usize;
			sync_committee_bits.get_mut(0..num_of_participants).unwrap().fill(true);
			update.sync_aggregate.sync_committee_bits =
				sync_committee_bits.as_raw_mut_slice().to_vec().into();
			assert_err!(
				Eth2Client::submit_beacon_chain_light_client_update(
					Origin::signed(ALICE),
					KILN_CHAIN,
					update
				),
				Error::<Test>::SyncCommitteeBitsSumLessThanThreshold,
			);
		});
	}

	#[test]
	pub fn test_panic_on_missing_sync_committee_update() {
		new_test_ext().execute_with(|| {
			let (_headers, updates, _init_input) = get_test_context(None);
			let mut update = updates[1].clone();
			update.sync_committee_update = None;

			assert_err!(
				Eth2Client::submit_beacon_chain_light_client_update(
					Origin::signed(ALICE),
					KILN_CHAIN,
					update
				),
				Error::<Test>::SyncCommitteeUpdateNotPresent
			);
		});
	}
}

mod mainnet_tests {
	use crate::{
		mock::{new_test_ext, Test},
		Error,
	};

	use super::*;

	#[test]
	pub fn test_panic_on_init_in_trustless_mode_without_bls_on_mainnet() {
		new_test_ext().execute_with(|| {
			let (_headers, _updates, init_input) = get_test_data(Some(InitOptions {
				validate_updates: true,
				verify_bls_signatures: false,
				hashes_gc_threshold: 500,
				max_submitted_blocks_by_account: 7000,
				trusted_signer: None,
			}));

			assert_err!(
				Eth2Client::init(Origin::signed(ALICE), MAINNET_CHAIN, Box::new(init_input)),
				Error::<Test>::TrustlessModeError,
			);
		})
	}
}
