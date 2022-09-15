// Copyright 2020-2022 Litentry Technologies GmbH.
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

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use crate::{BlockNumber, Metadata};

// The context associated with the (litentry-account, did) pair
// TODO: maybe we have better naming
#[derive(Clone, Eq, PartialEq, Default, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct IdentityContext {
	// the metadata
	pub metadata: Option<Metadata>,
	// the block number (of parent chain) where the linking was intially requested
	pub linking_request_block: BlockNumber,
	// if this did is verified
	pub is_verified: bool,
}

impl IdentityContext {
	pub fn new(linking_request_block: BlockNumber) -> Self {
		Self { metadata: None, linking_request_block, is_verified: false }
	}
}