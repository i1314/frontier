// SPDX-License-Identifier: Apache-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking
use sp_std::prelude::*;
use crate::{Config, Module, FeeCalculator,
	runner::Runner};
use frame_benchmarking::{benchmarks, account};
use sp_core::{U256, H160};
use rlp::RlpStream;
use sha3::{Keccak256, Digest};

benchmarks! {

	runner_execute {

		let x in 1..10000000;

		// contract bytecode below is for:
		//
		// pragma solidity >=0.8.0;
		//
		// contract InfiniteContractVar {
		//     uint public count;

		//     constructor() public {
		//         count = 0;
		//     }

		//     function infinite() public {
		//         while (true) {
		//             count=count+1;
		//         }
		//     }
		// }

		let contract_bytecode = hex::decode(concat!(
			"608060405234801561001057600080fd5b506000808190555061017c806100276000396000f3fe60",
			"8060405234801561001057600080fd5b50600436106100365760003560e01c806306661abd146100",
			"3b5780635bec9e6714610059575b600080fd5b610043610063565b604051610050919061009c565b",
			"60405180910390f35b610061610069565b005b60005481565b5b60011561008b5760016000546100",
			"8091906100b7565b60008190555061006a565b565b6100968161010d565b82525050565b60006020",
			"820190506100b1600083018461008d565b92915050565b60006100c28261010d565b91506100cd83",
			"61010d565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
			"ff0382111561010257610101610117565b5b828201905092915050565b6000819050919050565b7f",
			"4e487b71000000000000000000000000000000000000000000000000000000006000526011600452",
			"60246000fdfea2646970667358221220bcab0385167dbed28dee34f1c5b30ecfcd67915495f0a32d",
			"2eeada8e094193a364736f6c63430008030033"))
			.expect("Bad hex string");

		let caller = H160::default();

		let mut nonce: u64 = 0;
		let nonce_as_u256: U256 = nonce.into();

		let value = U256::default();
		let gas_limit_create: u64 = 1_250_000 * 1_000_000_000;
		let create_runner_results = T::Runner::create(
			caller,
			contract_bytecode,
			value,
			gas_limit_create,
			None,
			Some(nonce_as_u256),
			T::config(),
		);
		assert_eq!(create_runner_results.is_ok(), true, "create() failed");

		let mut rlp = RlpStream::new_list(2);
		rlp.append(&caller);
		rlp.append(&0u8);
		let contract_address = H160::from_slice(&Keccak256::digest(&rlp.out())[12..]);

		let mut encoded_call = vec![0u8; 4];
		encoded_call[0..4].copy_from_slice(&Keccak256::digest(b"infinite()")[0..4]);

		let gas_limit_call = x as u64;

	}: {

		nonce = nonce + 1;
		let nonce_as_u256: U256 = nonce.into();

		let call_runner_results = T::Runner::call(
			caller,
			contract_address,
			encoded_call,
			value,
			gas_limit_call,
			None,
			Some(nonce_as_u256),
			T::config(),
		);
		assert_eq!(call_runner_results.is_ok(), true, "call() failed");
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::Test;
	use frame_support::assert_ok;
	use sp_io::TestExternalities;

	pub fn new_test_ext() -> TestExternalities {
		let t = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap();
		TestExternalities::new(t)
	}

	#[test]
	fn test_runner_execute() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_runner_execute::<Test>());
		});
	}
}
