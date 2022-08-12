// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// Copyright (C) 2022 KOOMPI.
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

//! Private implementation details of Kumandra consensus digests.

use crate::{ConsensusLog, FarmerPublicKey, FarmerSignature, KUMANDRA_ENGINE_ID};
use codec::{Decode, Encode};
use sp_consensus_slots::Slot;
use sp_runtime::DigestItem;
use kumandra_core_primitives::{Randomness, Salt, Solution};

/// A Kumandra pre-runtime digest. This contains all data required to validate a block and for the
/// Kumandra runtime module.
#[derive(Debug, Clone, Encode, Decode)]
pub struct PreDigest<PublicKey, RewardAddress> {
    /// Slot
    pub slot: Slot,
    /// Solution (includes PoR)
    pub solution: Solution<PublicKey, RewardAddress>,
}

/// Information about the global randomness for the block.
#[derive(Debug, Decode, Encode, PartialEq, Eq, Clone)]
pub struct GlobalRandomnessDescriptor {
    /// Global randomness used for deriving global slot challenges.
    pub global_randomness: Randomness,
}

/// Information about the solution range for the block.
#[derive(Debug, Decode, Encode, PartialEq, Eq, Clone)]
pub struct SolutionRangeDescriptor {
    /// Solution range used for challenges.
    pub solution_range: u64,
}

/// Salt for the block.
#[derive(Debug, Decode, Encode, PartialEq, Eq, Clone)]
pub struct SaltDescriptor {
    /// Salt used with challenges.
    pub salt: Salt,
}

/// A digest item which is usable with Kumandra consensus.
pub trait CompatibleDigestItem: Sized {
    /// Construct a digest item which contains a Kumandra pre-digest.
    fn kumandra_pre_digest<AccountId: Encode>(
        pre_digest: &PreDigest<FarmerPublicKey, AccountId>,
    ) -> Self;

    /// If this item is an Kumandra pre-digest, return it.
    fn as_kumandra_pre_digest<AccountId: Decode>(
        &self,
    ) -> Option<PreDigest<FarmerPublicKey, AccountId>>;

    /// Construct a digest item which contains a Kumandra seal.
    fn kumandra_seal(signature: FarmerSignature) -> Self;

    /// If this item is a Kumandra signature, return the signature.
    fn as_kumandra_seal(&self) -> Option<FarmerSignature>;

    /// Construct a digest item which contains a global randomness descriptor.
    fn global_randomness_descriptor(global_randomness: GlobalRandomnessDescriptor) -> Self;

    /// If this item is a Kumandra global randomness descriptor, return it.
    fn as_global_randomness_descriptor(&self) -> Option<GlobalRandomnessDescriptor>;

    /// Construct a digest item which contains a solution range descriptor.
    fn solution_range_descriptor(solution_range: SolutionRangeDescriptor) -> Self;

    /// If this item is a Kumandra solution range descriptor, return it.
    fn as_solution_range_descriptor(&self) -> Option<SolutionRangeDescriptor>;

    /// Construct a digest item which contains a salt descriptor.
    fn salt_descriptor(salt: SaltDescriptor) -> Self;

    /// If this item is a Kumandra salt descriptor, return it.
    fn as_salt_descriptor(&self) -> Option<SaltDescriptor>;
}

impl CompatibleDigestItem for DigestItem {
    fn kumandra_pre_digest<RewardAddress: Encode>(
        pre_digest: &PreDigest<FarmerPublicKey, RewardAddress>,
    ) -> Self {
        Self::PreRuntime(KUMANDRA_ENGINE_ID, pre_digest.encode())
    }

    fn as_kumandra_pre_digest<RewardAddress: Decode>(
        &self,
    ) -> Option<PreDigest<FarmerPublicKey, RewardAddress>> {
        self.pre_runtime_try_to(&KUMANDRA_ENGINE_ID)
    }

    fn kumandra_seal(signature: FarmerSignature) -> Self {
        Self::Seal(KUMANDRA_ENGINE_ID, signature.encode())
    }

    fn as_kumandra_seal(&self) -> Option<FarmerSignature> {
        self.seal_try_to(&KUMANDRA_ENGINE_ID)
    }

    /// Construct a digest item which contains a global randomness descriptor.
    fn global_randomness_descriptor(global_randomness: GlobalRandomnessDescriptor) -> Self {
        Self::Consensus(
            KUMANDRA_ENGINE_ID,
            ConsensusLog::GlobalRandomness(global_randomness).encode(),
        )
    }

    /// If this item is a Kumandra global randomness descriptor, return it.
    fn as_global_randomness_descriptor(&self) -> Option<GlobalRandomnessDescriptor> {
        self.consensus_try_to(&KUMANDRA_ENGINE_ID).and_then(|c| {
            if let ConsensusLog::GlobalRandomness(global_randomness) = c {
                Some(global_randomness)
            } else {
                None
            }
        })
    }

    fn solution_range_descriptor(solution_range: SolutionRangeDescriptor) -> Self {
        Self::Consensus(
            KUMANDRA_ENGINE_ID,
            ConsensusLog::SolutionRange(solution_range).encode(),
        )
    }

    fn as_solution_range_descriptor(&self) -> Option<SolutionRangeDescriptor> {
        self.consensus_try_to(&KUMANDRA_ENGINE_ID).and_then(|c| {
            if let ConsensusLog::SolutionRange(solution_range) = c {
                Some(solution_range)
            } else {
                None
            }
        })
    }

    fn salt_descriptor(salt: SaltDescriptor) -> Self {
        Self::Consensus(KUMANDRA_ENGINE_ID, ConsensusLog::Salt(salt).encode())
    }

    fn as_salt_descriptor(&self) -> Option<SaltDescriptor> {
        self.consensus_try_to(&KUMANDRA_ENGINE_ID).and_then(|c| {
            if let ConsensusLog::Salt(salt) = c {
                Some(salt)
            } else {
                None
            }
        })
    }
}
