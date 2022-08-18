// Copyright (C) 2022 KOOMPI Inc.
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

//! Core primitives for Kumandra Network.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, missing_docs)]
#![cfg_attr(feature = "std", warn(missing_debug_implementations))]
#![feature(int_log)]

#[cfg(test)]
mod tests;

pub mod crypto;
pub mod objects;

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::convert::AsRef;
use core::ops::{Deref, DerefMut};
use derive_more::{Add, Display, Div, Mul, Sub};
use num_traits::{WrappingAdd, WrappingSub};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Size of Sha2-256 hash output (in bytes)
pub const SHA256_HASH_SIZE: usize = 32;

/// Byte size of a piece in Kumandra Network, 4KiB.
///
/// This can not changed after the network is launched.
pub const PIECE_SIZE: usize = 4096;

/// Byte length of a randomness type.
pub const RANDOMNESS_LENGTH: usize = 32;

/// Sha2-256 hash output
pub type Sha256Hash = [u8; SHA256_HASH_SIZE];

/// Type of randomness.
pub type Randomness = [u8; RANDOMNESS_LENGTH];

/// Size of `Tag` in bytes.
pub const TAG_SIZE: usize = 8;

/// Type of the commitment for a particular piece.
pub type Tag = [u8; TAG_SIZE];

/// Tag prefix
pub const SALT_HASHING_PREFIX: &[u8] = b"salt";

/// Size of `Tag` in bytes.
pub const SALT_SIZE: usize = 8;

/// Salt used for creating commitment tags for pieces.
pub type Salt = [u8; SALT_SIZE];

/// Block number in Kumandra network.
pub type BlockNumber = u32;

/// Slot number in Kumandra network.
pub type SlotNumber = u64;

/// Type of solution range.
pub type SolutionRange = u64;

/// BlockWeight type for fork choice rules.
///
/// The closer solution's tag is to the target, the heavier it is.
pub type BlockWeight = u128;

/// Segment index type.
pub type SegmentIndex = u64;

/// Records root type.
pub type RecordsRoot = Sha256Hash;

/// Eon Index type.
pub type EonIndex = u64;

/// Length of public key in bytes.
pub const PUBLIC_KEY_LENGTH: usize = 32;

/// 128 data records and 128 parity records (as a result of erasure coding) together form a perfect
/// Merkle Tree and will result in witness size of `log2(MERKLE_NUM_LEAVES) * SHA256_HASH_SIZE`.
///
/// This number is a tradeoff:
/// * as this number goes up, fewer [`RootBlock`]s are required to be stored for verifying archival
///   history of the network, which makes sync quicker and more efficient, but also more data in
///   each [`Piece`] will be occupied with witness, thus wasting space that otherwise could have
///   been used for storing data (record part of a Piece)
/// * as this number goes down, witness get smaller leading to better piece utilization, but the
///   number of root blocks goes up making sync less efficient and less records are needed to be
///   lost before part of the archived history become unrecoverable, reducing reliability of the
///   data stored on the network
pub const MERKLE_NUM_LEAVES: u32 = 256;
/// Size of witness for a segment record (in bytes).
pub const WITNESS_SIZE: u32 = SHA256_HASH_SIZE as u32 * MERKLE_NUM_LEAVES.log2();
/// Size of a segment record given the global piece size (in bytes).
pub const RECORD_SIZE: u32 = PIECE_SIZE as u32 - WITNESS_SIZE;
/// Recorded History Segment Size includes half of the records (just data records) that will later
/// be erasure coded and together with corresponding witnesses will result in `MERKLE_NUM_LEAVES`
/// pieces of archival history.
pub const RECORDED_HISTORY_SEGMENT_SIZE: u32 = RECORD_SIZE * MERKLE_NUM_LEAVES / 2;

/// Randomness context
pub const RANDOMNESS_CONTEXT: &[u8] = b"kumandra_randomness";

/// Length of signature in bytes
pub const REWARD_SIGNATURE_LENGTH: usize = 64;
const VRF_OUTPUT_LENGTH: usize = 32;
const VRF_PROOF_LENGTH: usize = 64;

/// A Ristretto Schnorr public key as bytes produced by `schnorrkel` crate.
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode, TypeInfo,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PublicKey(
    #[cfg_attr(feature = "std", serde(with = "hex::serde"))] [u8; PUBLIC_KEY_LENGTH],
);

impl From<[u8; PUBLIC_KEY_LENGTH]> for PublicKey {
    fn from(bytes: [u8; PUBLIC_KEY_LENGTH]) -> Self {
        Self(bytes)
    }
}

impl From<PublicKey> for [u8; PUBLIC_KEY_LENGTH] {
    fn from(public_key: PublicKey) -> Self {
        public_key.0
    }
}

impl Deref for PublicKey {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// A Ristretto Schnorr signature as bytes produced by `schnorrkel` crate.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RewardSignature(
    #[cfg_attr(feature = "std", serde(with = "serde_arrays"))] [u8; REWARD_SIGNATURE_LENGTH],
);

impl From<[u8; REWARD_SIGNATURE_LENGTH]> for RewardSignature {
    fn from(bytes: [u8; REWARD_SIGNATURE_LENGTH]) -> Self {
        Self(bytes)
    }
}

impl From<RewardSignature> for [u8; REWARD_SIGNATURE_LENGTH] {
    fn from(signature: RewardSignature) -> Self {
        signature.0
    }
}

impl Deref for RewardSignature {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for RewardSignature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// VRF signature output and proof as produced by `schnorrkel` crate.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TagSignature {
    /// VRF output bytes.
    pub output: [u8; VRF_OUTPUT_LENGTH],
    /// VRF proof bytes.
    #[cfg_attr(feature = "std", serde(with = "serde_arrays"))]
    pub proof: [u8; VRF_PROOF_LENGTH],
}

/// VRF signature output and proof as produced by `schnorrkel` crate.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct LocalChallenge {
    /// VRF output bytes.
    pub output: [u8; VRF_OUTPUT_LENGTH],
    /// VRF proof bytes.
    #[cfg_attr(feature = "std", serde(with = "serde_arrays"))]
    pub proof: [u8; VRF_PROOF_LENGTH],
}

/// A piece of archival history in Kumandra Network.
///
/// Internally piece contains a record and corresponding witness that together with [`RootBlock`] of
/// the segment this piece belongs to can be used to verify that a piece belongs to the actual
/// archival history of the blockchain.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Piece(Vec<u8>);

impl Default for Piece {
    fn default() -> Self {
        Self(vec![0u8; PIECE_SIZE])
    }
}

impl From<[u8; PIECE_SIZE]> for Piece {
    fn from(piece: [u8; PIECE_SIZE]) -> Self {
        Self(piece.to_vec())
    }
}

impl From<Piece> for Vec<u8> {
    fn from(piece: Piece) -> Self {
        piece.0
    }
}

impl TryFrom<&[u8]> for Piece {
    type Error = &'static str;
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() != PIECE_SIZE {
            Err("Wrong piece size, expected: 4096")
        } else {
            Ok(Self(slice.to_vec()))
        }
    }
}

impl Deref for Piece {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Piece {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for Piece {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Piece {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

/// Flat representation of multiple pieces concatenated for higher efficient for processing.
#[derive(Debug, Default, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FlatPieces(Vec<u8>);

impl FlatPieces {
    /// Allocate `FlatPieces` that will hold `piece_count` pieces filled with zeroes.
    pub fn new(piece_count: usize) -> Self {
        Self(vec![0u8; piece_count * PIECE_SIZE])
    }

    /// Number of pieces contained.
    pub fn count(&self) -> usize {
        self.0.len() / PIECE_SIZE
    }

    /// Extract internal flat representation of bytes.
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }

    /// Iterator over individual pieces as byte slices.
    pub fn as_pieces(&self) -> impl ExactSizeIterator<Item = &[u8]> {
        self.0.chunks_exact(PIECE_SIZE)
    }

    /// Iterator over individual pieces as byte slices.
    pub fn as_pieces_mut(&mut self) -> impl ExactSizeIterator<Item = &mut [u8]> {
        self.0.chunks_exact_mut(PIECE_SIZE)
    }
}

impl From<Piece> for FlatPieces {
    fn from(Piece(piece): Piece) -> Self {
        Self(piece)
    }
}

// TODO: Remove once we no longer use `unzip` in farmer and get `(Vec<PieceIndex>, FlatPieces)`
// after requesting sequential pieces.
impl Extend<Piece> for FlatPieces {
    fn extend<T: IntoIterator<Item = Piece>>(&mut self, iter: T) {
        self.0
            .extend(iter.into_iter().flat_map(|piece| piece.0.into_iter()))
    }
}

impl TryFrom<Vec<u8>> for FlatPieces {
    type Error = Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() % PIECE_SIZE != 0 {
            return Err(value);
        }

        Ok(Self(value))
    }
}

impl Deref for FlatPieces {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FlatPieces {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for FlatPieces {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for FlatPieces {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

/// Progress of an archived block.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum ArchivedBlockProgress {
    /// The block has been fully archived.
    Complete,

    /// Number of paritally archived bytes of a block.
    Partial(u32),
}

impl Default for ArchivedBlockProgress {
    /// We assume a block can always fit into the segment initially, but it can definitely possible
    /// to be transitioned into the partial state after some overflow checkings.
    fn default() -> Self {
        Self::Complete
    }
}

impl ArchivedBlockProgress {
    /// Return the number of partially archived bytes if the progress is not complete.
    pub fn partial(&self) -> Option<u32> {
        match self {
            Self::Complete => None,
            Self::Partial(number) => Some(*number),
        }
    }

    /// Sets new number of partially archived bytes.
    pub fn set_partial(&mut self, new_partial: u32) {
        *self = Self::Partial(new_partial);
    }
}

/// Last archived block
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct LastArchivedBlock {
    /// Block number
    pub number: u32,
    /// Progress of an archived block.
    pub archived_progress: ArchivedBlockProgress,
}

impl LastArchivedBlock {
    /// Returns the number of partially archived bytes for a block.
    pub fn partial_archived(&self) -> Option<u32> {
        self.archived_progress.partial()
    }

    /// Sets new number of partially archived bytes.
    pub fn set_partial_archived(&mut self, new_partial: u32) {
        self.archived_progress.set_partial(new_partial);
    }

    /// Sets the archived state of this block to [`ArchivedBlockProgress::Complete`].
    pub fn set_complete(&mut self) {
        self.archived_progress = ArchivedBlockProgress::Complete;
    }
}

/// Root block for a specific segment.
///
/// Each segment will have corresponding [`RootBlock`] included as the first item in the next
/// segment. Each `RootBlock` includes hash of the previous one and all together form a chain of
/// root blocks that is used for quick and efficient verification that some [`Piece`] corresponds to
/// the actual archival history of the blockchain.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum RootBlock {
    /// V0 of the root block data structure
    #[codec(index = 0)]
    #[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
    V0 {
        /// Segment index
        segment_index: SegmentIndex,
        /// Merkle root of the records in a segment.
        records_root: RecordsRoot,
        /// Hash of the root block of the previous segment
        prev_root_block_hash: Sha256Hash,
        /// Last archived block
        last_archived_block: LastArchivedBlock,
    },
}

impl RootBlock {
    /// Hash of the whole root block
    pub fn hash(&self) -> Sha256Hash {
        crypto::sha256_hash(&self.encode())
    }

    /// Segment index
    pub fn segment_index(&self) -> u64 {
        match self {
            Self::V0 { segment_index, .. } => *segment_index,
        }
    }

    /// Merkle root of the records in a segment.
    pub fn records_root(&self) -> RecordsRoot {
        match self {
            Self::V0 { records_root, .. } => *records_root,
        }
    }

    /// Hash of the root block of the previous segment
    pub fn prev_root_block_hash(&self) -> Sha256Hash {
        match self {
            Self::V0 {
                prev_root_block_hash,
                ..
            } => *prev_root_block_hash,
        }
    }

    /// Last archived block
    pub fn last_archived_block(&self) -> LastArchivedBlock {
        match self {
            Self::V0 {
                last_archived_block,
                ..
            } => *last_archived_block,
        }
    }
}

/// Piece index in consensus
pub type PieceIndex = u64;

/// Hash of `PieceIndex`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Decode, Encode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PieceIndexHash(Sha256Hash);

impl From<PieceIndexHash> for Sha256Hash {
    fn from(piece_index_hash: PieceIndexHash) -> Self {
        piece_index_hash.0
    }
}

impl From<Sha256Hash> for PieceIndexHash {
    fn from(hash: Sha256Hash) -> Self {
        Self(hash)
    }
}

impl AsRef<[u8]> for PieceIndexHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl PieceIndexHash {
    /// Constructs `PieceIndexHash` from `PieceIndex`
    pub fn from_index(index: PieceIndex) -> Self {
        Self(crypto::sha256_hash(&index.to_le_bytes()))
    }
}

// TODO: Versioned solution enum
/// Farmer solution for slot challenge.
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Solution<PublicKey, RewardAddress> {
    /// Public key of the farmer that created the solution
    pub public_key: PublicKey,
    /// Address for receiving block reward
    pub reward_address: RewardAddress,
    /// Index of encoded piece
    pub piece_index: PieceIndex,
    /// Encoding
    pub encoding: Piece,
    /// VRF signature of the tag
    pub tag_signature: TagSignature,
    /// Local challenge derived from global challenge using farmer's identity.
    pub local_challenge: LocalChallenge,
    /// Tag (hmac of encoding and salt)
    pub tag: Tag,
}

impl<PublicKey, RewardAddressA> Solution<PublicKey, RewardAddressA> {
    /// Transform solution with one reward address type into solution with another compatible
    /// reward address type.
    pub fn into_reward_address_format<T, RewardAddressB>(
        self,
    ) -> Solution<PublicKey, RewardAddressB>
    where
        RewardAddressA: Into<T>,
        T: Into<RewardAddressB>,
    {
        let Solution {
            public_key,
            reward_address,
            piece_index,
            encoding,
            tag_signature,
            local_challenge,
            tag,
        } = self;
        Solution {
            public_key,
            reward_address: Into::<T>::into(reward_address).into(),
            piece_index,
            encoding,
            tag_signature,
            local_challenge,
            tag,
        }
    }
}

impl<PublicKey, RewardAddress> Solution<PublicKey, RewardAddress>
where
    PublicKey: Clone,
    RewardAddress: Clone,
{
    /// Dummy solution for the genesis block
    pub fn genesis_solution(public_key: PublicKey, reward_address: RewardAddress) -> Self {
        Self {
            public_key,
            reward_address,
            piece_index: 0,
            encoding: Piece::default(),
            tag_signature: TagSignature {
                output: [0; 32],
                proof: [0; 64],
            },
            local_challenge: LocalChallenge {
                output: [0; 32],
                proof: [0; 64],
            },
            tag: Tag::default(),
        }
    }
}

/// Bidirectional distance metric implemented on top of subtraction
pub fn bidirectional_distance<T: num_traits::WrappingSub + Ord>(a: &T, b: &T) -> T {
    let diff = a.wrapping_sub(b);
    let diff2 = b.wrapping_sub(a);
    // Find smaller diff between 2 directions.
    diff.min(diff2)
}

#[allow(clippy::assign_op_pattern, clippy::ptr_offset_with_cast)]
mod private_u256 {
    //! This module is needed to scope clippy allows

    uint::construct_uint! {
        pub struct U256(4);
    }
}

/// 256-bit unsigned integer
#[derive(Debug, Display, Add, Sub, Mul, Div, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct U256(private_u256::U256);

impl U256 {
    /// Zero (additive identity) of this type.
    pub const fn zero() -> Self {
        Self(private_u256::U256::zero())
    }

    /// One (multiplicative identity) of this type.
    pub fn one() -> Self {
        Self(private_u256::U256::one())
    }

    /// Create from big endian bytes
    pub fn from_be_bytes(bytes: [u8; 32]) -> Self {
        Self(private_u256::U256::from_big_endian(&bytes))
    }

    /// Convert to big endian bytes
    pub fn to_be_bytes(self) -> [u8; 32] {
        let mut arr = [0u8; 32];
        self.0.to_big_endian(&mut arr);
        arr
    }

    /// Create from little endian bytes
    pub fn from_le_bytes(bytes: [u8; 32]) -> Self {
        Self(private_u256::U256::from_little_endian(&bytes))
    }

    /// Convert to little endian bytes
    pub fn to_le_bytes(self) -> [u8; 32] {
        let mut arr = [0u8; 32];
        self.0.to_little_endian(&mut arr);
        arr
    }

    /// Adds two numbers, checking for overflow. If overflow happens, `None` is returned.
    pub fn checked_add(&self, v: &Self) -> Option<Self> {
        self.0.checked_add(v.0).map(Self)
    }

    /// Subtracts two numbers, checking for underflow. If underflow happens, `None` is returned.
    pub fn checked_sub(&self, v: &Self) -> Option<Self> {
        self.0.checked_sub(v.0).map(Self)
    }

    /// Multiplies two numbers, checking for underflow or overflow. If underflow or overflow
    /// happens, `None` is returned.
    pub fn checked_mul(&self, v: &Self) -> Option<Self> {
        self.0.checked_mul(v.0).map(Self)
    }

    /// Divides two numbers, checking for underflow, overflow and division by zero. If any of that
    /// happens, `None` is returned.
    pub fn checked_div(&self, v: &Self) -> Option<Self> {
        self.0.checked_div(v.0).map(Self)
    }

    /// Saturating addition. Computes `self + other`, saturating at the relevant high or low
    /// boundary of the type.
    pub fn saturating_add(&self, v: &Self) -> Self {
        Self(self.0.saturating_add(v.0))
    }

    /// Saturating subtraction. Computes `self - other`, saturating at the relevant high or low
    /// boundary of the type.
    pub fn saturating_sub(&self, v: &Self) -> Self {
        Self(self.0.saturating_sub(v.0))
    }

    /// Saturating multiplication. Computes `self * other`, saturating at the relevant high or low
    /// boundary of the type.
    pub fn saturating_mul(&self, v: &Self) -> Self {
        Self(self.0.saturating_mul(v.0))
    }

    /// The middle of the piece distance field.
    /// The analogue of `0b1000_0000` for `u8`.
    pub const MIDDLE: Self = {
        // TODO: This assumes that numbers are stored little endian,
        //  should be replaced with just `Self::MAX / 2`, but it is not `const fn` in Rust yet.
        Self(private_u256::U256([
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX / 2,
        ]))
    };

    /// Maximum value.
    pub const MAX: Self = Self(private_u256::U256::MAX);
}

// Necessary for division derive
impl From<U256> for private_u256::U256 {
    fn from(number: U256) -> Self {
        number.0
    }
}

impl WrappingAdd for U256 {
    fn wrapping_add(&self, other: &Self) -> Self {
        Self(self.0.overflowing_add(other.0).0)
    }
}

impl WrappingSub for U256 {
    fn wrapping_sub(&self, other: &Self) -> Self {
        Self(self.0.overflowing_sub(other.0).0)
    }
}

impl From<u8> for U256 {
    fn from(number: u8) -> Self {
        Self(number.into())
    }
}

impl From<u16> for U256 {
    fn from(number: u16) -> Self {
        Self(number.into())
    }
}

impl From<u32> for U256 {
    fn from(number: u32) -> Self {
        Self(number.into())
    }
}

impl From<u64> for U256 {
    fn from(number: u64) -> Self {
        Self(number.into())
    }
}

impl From<u128> for U256 {
    fn from(number: u128) -> Self {
        Self(number.into())
    }
}

impl From<PieceIndexHash> for U256 {
    fn from(PieceIndexHash(hash): PieceIndexHash) -> Self {
        Self(private_u256::U256::from_big_endian(&hash))
    }
}

impl From<U256> for PieceIndexHash {
    fn from(number: U256) -> Self {
        Self(number.to_be_bytes())
    }
}
