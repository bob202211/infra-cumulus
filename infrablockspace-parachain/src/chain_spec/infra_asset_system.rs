// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use crate::chain_spec::{
	get_account_id_from_seed, get_collator_keys_from_seed, Extensions, SAFE_XCM_VERSION,
};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachains_common::{AccountId, AuraId, Balance as InfraAssetSystemBalance};
use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type InfraAssetSystemChainSpec =
	sc_service::GenericChainSpec<infra_asset_system_runtime::GenesisConfig, Extensions>;

const INFRA_ASSET_SYSTEM_ED: InfraAssetSystemBalance =
	infra_asset_system_runtime::constants::currency::EXISTENTIAL_DEPOSIT;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn infra_asset_system_session_keys(keys: AuraId) -> infra_asset_system_runtime::SessionKeys {
	infra_asset_system_runtime::SessionKeys { aura: keys }
}

pub fn infra_asset_system_development_config() -> InfraAssetSystemChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 0.into());
	properties.insert("tokenSymbol".into(), "".into());
	properties.insert("tokenDecimals".into(), 10.into());

	InfraAssetSystemChainSpec::from_genesis(
		// Name
		"Infra Asset System Development",
		// ID
		"infra_asset_system_dev",
		ChainType::Local,
		move || {
			infra_asset_system_genesis(
				// initial collators.
				vec![(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed::<AuraId>("Alice"),
				)],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "polkadot-dev".into(), para_id: 1000 }, // ToDo: polkadot-dev -> infrabs-dev
	)
}

pub fn infra_asset_system_local_config() -> InfraAssetSystemChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 0.into());
	properties.insert("tokenSymbol".into(), "".into());
	properties.insert("tokenDecimals".into(), 10.into());

	InfraAssetSystemChainSpec::from_genesis(
		// Name
		"Infra Asset System Local",
		// ID
		"infra_asset_system_local",
		ChainType::Local,
		move || {
			infra_asset_system_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed::<AuraId>("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed::<AuraId>("Bob"),
					),
				],
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
				],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "infrablockspace-local".into(), para_id: 1000 },
	)
}

// Not used for syncing, but just to determine the genesis values set for the upgrade from shell.
pub fn infra_asset_system_config() -> InfraAssetSystemChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 0.into());
	properties.insert("tokenSymbol".into(), "DOT".into());
	properties.insert("tokenDecimals".into(), 10.into());

	InfraAssetSystemChainSpec::from_genesis(
		// Name
		"Infra Asset System",
		// ID
		"infra-asset-system",
		ChainType::Live,
		move || {
			infra_asset_system_genesis(
				// initial collators.
				vec![
					(
						hex!("4c3d674d2a01060f0ded218e5dcc6f90c1726f43df79885eb3e22d97a20d5421")
							.into(),
						hex!("4c3d674d2a01060f0ded218e5dcc6f90c1726f43df79885eb3e22d97a20d5421")
							.unchecked_into(),
					),
					(
						hex!("c7d7d38d16bc23c6321152c50306212dc22c0efc04a2e52b5cccfc31ab3d7811")
							.into(),
						hex!("c7d7d38d16bc23c6321152c50306212dc22c0efc04a2e52b5cccfc31ab3d7811")
							.unchecked_into(),
					),
					(
						hex!("c5c07ba203d7375675f5c1ebe70f0a5bb729ae57b48bcc877fcc2ab21309b762")
							.into(),
						hex!("c5c07ba203d7375675f5c1ebe70f0a5bb729ae57b48bcc877fcc2ab21309b762")
							.unchecked_into(),
					),
					(
						hex!("0b2d0013fb974794bd7aa452465b567d48ef70373fe231a637c1fb7c547e85b3")
							.into(),
						hex!("0b2d0013fb974794bd7aa452465b567d48ef70373fe231a637c1fb7c547e85b3")
							.unchecked_into(),
					),
				],
				vec![],
				1000u32.into(),
			)
		},
		vec![
			"/ip4/34.65.251.121/tcp/30334/p2p/12D3KooWG3GrM6XKMM4gp3cvemdwUvu96ziYoJmqmetLZBXE8bSa".parse().unwrap(),
			"/ip4/34.65.35.228/tcp/30334/p2p/12D3KooWMRyTLrCEPcAQD6c4EnudL3vVzg9zji3whvsMYPUYevpq".parse().unwrap(),
			"/ip4/34.83.247.146/tcp/30334/p2p/12D3KooWE4jFh5FpJDkWVZhnWtFnbSqRhdjvC7Dp9b8b3FTuubQC".parse().unwrap(),
			"/ip4/104.199.117.230/tcp/30334/p2p/12D3KooWG9R8pVXKumVo2rdkeVD4j5PVhRTqmYgLHY3a4yPYgLqM".parse().unwrap(),
		],
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "infrablockspace".into(), para_id: 1000 },
	)
}

pub fn infra_asset_system_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> infra_asset_system_runtime::GenesisConfig {
	let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

	infra_asset_system_runtime::GenesisConfig {
		system: infra_asset_system_runtime::SystemConfig {
			code: infra_asset_system_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: infra_asset_system_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, INFRA_ASSET_SYSTEM_ED * 4096))
				.collect(),
		},
		parachain_info: infra_asset_system_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: infra_asset_system_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: INFRA_ASSET_SYSTEM_ED * 16,
			..Default::default()
		},
		session: infra_asset_system_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                           // account id
						acc,                                   // validator id
						infra_asset_system_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		assets: pallet_assets::GenesisConfig {
			assets: vec![(
				99,                                                   // asset_id
				get_account_id_from_seed::<sr25519::Public>("Alice"), // owner
				true,                                                 // is_sufficient
				1000,                                                 // min_balance
			)],
			metadata: vec![(99, "iTEST".into(), "iTEST".into(), 12)],
			accounts: vec![(
				99,
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				1_000_000_000_000_000_000_000, // 1_000_000_000 iTEST
			)],
			..Default::default()
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		sudo: infra_asset_system_runtime::SudoConfig { key: Some(root_key) },
		parachain_system: Default::default(),
		infrablockspace_xcm: infra_asset_system_runtime::InfrablockspaceXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
	}
}
