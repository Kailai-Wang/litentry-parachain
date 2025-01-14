// Copyright 2020-2023 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

mod ethereum_signature;
mod identity;
// mod trusted_call;
mod assertion;
mod enclave_quote;
mod validation_data;

pub use ethereum_signature::*;
pub use identity::*;
pub use parentchain_primitives::{
	AesOutput, BlockNumber as ParentchainBlockNumber, UserShieldingKeyType, MINUTES,
	USER_SHIELDING_KEY_LEN, USER_SHIELDING_KEY_NONCE_LEN, USER_SHIELDING_KEY_TAG_LEN,
};

use ring::{
	aead::{Aad, BoundKey, Nonce, NonceSequence, SealingKey, UnboundKey, AES_256_GCM},
	error::Unspecified,
};

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate rand_sgx as rand;

use rand::Rng;

// pub use trusted_call::*;
pub use assertion::*;
pub use enclave_quote::*;
pub use validation_data::*;

pub const CHALLENGE_CODE_SIZE: usize = 16;
pub type ChallengeCode = [u8; CHALLENGE_CODE_SIZE];

pub fn aes_encrypt_default(key: &UserShieldingKeyType, data: &[u8]) -> AesOutput {
	let mut in_out = data.to_vec();

	let nonce = RingAeadNonceSequence::new();
	let aad = b"";
	let unbound_key = UnboundKey::new(&AES_256_GCM, key.as_slice()).unwrap();
	let mut sealing_key = SealingKey::new(unbound_key, nonce.clone());
	sealing_key.seal_in_place_append_tag(Aad::from(aad), &mut in_out).unwrap();

	AesOutput { ciphertext: in_out.to_vec(), aad: aad.to_vec(), nonce: nonce.nonce }
}

#[derive(Clone)]
pub struct RingAeadNonceSequence {
	pub nonce: [u8; USER_SHIELDING_KEY_NONCE_LEN],
}

impl RingAeadNonceSequence {
	fn new() -> RingAeadNonceSequence {
		RingAeadNonceSequence { nonce: [0u8; USER_SHIELDING_KEY_NONCE_LEN] }
	}
}

impl NonceSequence for RingAeadNonceSequence {
	fn advance(&mut self) -> Result<Nonce, Unspecified> {
		let nonce = Nonce::assume_unique_for_key(self.nonce);

		// FIXME: in function `ring::rand::sysrand::fill': undefined reference to `syscall'
		// let mut nonce_vec = vec![0; USER_SHIELDING_KEY_NONCE_LEN];
		// let rand = SystemRandom::new();
		// rand.fill(&mut nonce_vec).unwrap();
		let nonce_vec = rand::thread_rng().gen::<[u8; USER_SHIELDING_KEY_NONCE_LEN]>();

		self.nonce.copy_from_slice(&nonce_vec[0..USER_SHIELDING_KEY_NONCE_LEN]);

		Ok(nonce)
	}
}
