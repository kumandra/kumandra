// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// Copyright (C) 2022 Kumandra, Inc.
// SPDX-License-Identifier: Apache-2.0

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

//! Inherents for Kumandra consensus

use codec::{Decode, Encode};
use sp_consensus_slots::Slot;
use sp_inherents::{Error, InherentData, InherentIdentifier, IsFatalError};
use sp_runtime::RuntimeDebug;
use sp_std::result::Result;
use sp_std::vec::Vec;
use kumandra_core_primitives::RootBlock;

/// The Kumandra inherent identifier.
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"kumandra";

/// Errors that can occur while checking root blocks.
#[derive(Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub enum InherentError {
    /// List of root blocks is not correct.
    IncorrectRootBlocksList,
}

impl IsFatalError for InherentError {
    fn is_fatal_error(&self) -> bool {
        true
    }
}

/// The type of the Kumandra inherent data.
#[derive(Encode, Decode, RuntimeDebug)]
pub struct InherentType {
    /// Slot at which block was created.
    pub slot: Slot,
    /// Root blocks expected to be included in the block.
    pub root_blocks: Vec<RootBlock>,
}

/// Auxiliary trait to extract Kumandra inherent data.
pub trait KumandraInherentData {
    /// Get Kumandra inherent data.
    fn kumandra_inherent_data(&self) -> Result<Option<InherentType>, Error>;

    /// Replace Kumandra inherent data.
    fn replace_kumandra_inherent_data(&mut self, new: InherentType);
}

impl KumandraInherentData for InherentData {
    fn kumandra_inherent_data(&self) -> Result<Option<InherentType>, Error> {
        self.get_data(&INHERENT_IDENTIFIER)
    }

    fn replace_kumandra_inherent_data(&mut self, new: InherentType) {
        self.replace_data(INHERENT_IDENTIFIER, &new);
    }
}

/// Provides the root blocks inherent data for Kumandra.
#[cfg(feature = "std")]
pub struct InherentDataProvider {
    data: InherentType,
}

#[cfg(feature = "std")]
impl InherentDataProvider {
    /// Create new inherent data provider from the given `data`.
    pub fn new(slot: Slot, root_blocks: Vec<RootBlock>) -> Self {
        Self {
            data: InherentType { slot, root_blocks },
        }
    }

    /// Creates the inherent data provider by calculating the slot from the given
    /// `timestamp` and `duration`.
    pub fn from_timestamp_and_slot_duration(
        timestamp: sp_timestamp::Timestamp,
        slot_duration: sp_consensus_slots::SlotDuration,
        root_blocks: Vec<RootBlock>,
    ) -> Self {
        let slot = Slot::from_timestamp(timestamp, slot_duration);

        Self::new(slot, root_blocks)
    }

    /// Returns the `data` of this inherent data provider.
    pub fn data(&self) -> &InherentType {
        &self.data
    }
}

#[cfg(feature = "std")]
impl sp_std::ops::Deref for InherentDataProvider {
    type Target = Slot;

    fn deref(&self) -> &Self::Target {
        &self.data.slot
    }
}

#[cfg(feature = "std")]
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
    fn provide_inherent_data(&self, inherent_data: &mut InherentData) -> Result<(), Error> {
        inherent_data.put_data(INHERENT_IDENTIFIER, &self.data)
    }

    async fn try_handle_error(
        &self,
        identifier: &InherentIdentifier,
        error: &[u8],
    ) -> Option<Result<(), Error>> {
        if *identifier != INHERENT_IDENTIFIER {
            return None;
        }

        let error = InherentError::decode(&mut &*error).ok()?;

        Some(Err(Error::Application(Box::from(format!("{:?}", error)))))
    }
}// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// Copyright (C) 2022 Kumandra, Inc.
// SPDX-License-Identifier: Apache-2.0

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

//! Inherents for Kumandra consensus

use codec::{Decode, Encode};
use sp_consensus_slots::Slot;
use sp_inherents::{Error, InherentData, InherentIdentifier, IsFatalError};
use sp_runtime::RuntimeDebug;
use sp_std::result::Result;
use sp_std::vec::Vec;
use kumandra_core_primitives::RootBlock;

/// The Kumandra inherent identifier.
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"kumandra";

/// Errors that can occur while checking root blocks.
#[derive(Encode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Decode))]
pub enum InherentError {
    /// List of root blocks is not correct.
    IncorrectRootBlocksList,
}

impl IsFatalError for InherentError {
    fn is_fatal_error(&self) -> bool {
        true
    }
}

/// The type of the Kumandra inherent data.
#[derive(Encode, Decode, RuntimeDebug)]
pub struct InherentType {
    /// Slot at which block was created.
    pub slot: Slot,
    /// Root blocks expected to be included in the block.
    pub root_blocks: Vec<RootBlock>,
}

/// Auxiliary trait to extract Kumandra inherent data.
pub trait KumandraInherentData {
    /// Get Kumandra inherent data.
    fn kumandra_inherent_data(&self) -> Result<Option<InherentType>, Error>;

    /// Replace Kumandra inherent data.
    fn replace_kumandra_inherent_data(&mut self, new: InherentType);
}

impl KumandraInherentData for InherentData {
    fn kumandra_inherent_data(&self) -> Result<Option<InherentType>, Error> {
        self.get_data(&INHERENT_IDENTIFIER)
    }

    fn replace_kumandra_inherent_data(&mut self, new: InherentType) {
        self.replace_data(INHERENT_IDENTIFIER, &new);
    }
}

/// Provides the root blocks inherent data for Kumandra.
#[cfg(feature = "std")]
pub struct InherentDataProvider {
    data: InherentType,
}

#[cfg(feature = "std")]
impl InherentDataProvider {
    /// Create new inherent data provider from the given `data`.
    pub fn new(slot: Slot, root_blocks: Vec<RootBlock>) -> Self {
        Self {
            data: InherentType { slot, root_blocks },
        }
    }

    /// Creates the inherent data provider by calculating the slot from the given
    /// `timestamp` and `duration`.
    pub fn from_timestamp_and_slot_duration(
        timestamp: sp_timestamp::Timestamp,
        slot_duration: sp_consensus_slots::SlotDuration,
        root_blocks: Vec<RootBlock>,
    ) -> Self {
        let slot = Slot::from_timestamp(timestamp, slot_duration);

        Self::new(slot, root_blocks)
    }

    /// Returns the `data` of this inherent data provider.
    pub fn data(&self) -> &InherentType {
        &self.data
    }
}

#[cfg(feature = "std")]
impl sp_std::ops::Deref for InherentDataProvider {
    type Target = Slot;

    fn deref(&self) -> &Self::Target {
        &self.data.slot
    }
}

#[cfg(feature = "std")]
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for InherentDataProvider {
    fn provide_inherent_data(&self, inherent_data: &mut InherentData) -> Result<(), Error> {
        inherent_data.put_data(INHERENT_IDENTIFIER, &self.data)
    }

    async fn try_handle_error(
        &self,
        identifier: &InherentIdentifier,
        error: &[u8],
    ) -> Option<Result<(), Error>> {
        if *identifier != INHERENT_IDENTIFIER {
            return None;
        }

        let error = InherentError::decode(&mut &*error).ok()?;

        Some(Err(Error::Application(Box::from(format!("{:?}", error)))))
    }
}