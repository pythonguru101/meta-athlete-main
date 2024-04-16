#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;

pub type Balance = u128;
pub type AthleteId = u64;
pub type InstanceId = u32;

pub const CARD_HASH_KEY: &str = "meta_athlete_nft";

/// A reference to a piece of data stored off-chain.
///
/// Internally this is a 256-bit BLAKE2 hash of the data, which can be used to retrieve and verify the data
/// from an off-chain source, such as cloud storage or a node, without requiring the data
/// itself to be stored on-chain.
#[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
pub struct OffchainRef {
  pub hash: H256,
}

#[derive(Clone, Copy, Debug, PartialEq, Decode, Encode, TypeInfo)]
pub struct CardId {
  pub athlete_id: AthleteId,
  pub instance_id: InstanceId,
}

// /// A birthdate internally stored as an integer representing YMD.
// #[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
// pub struct Birthdate {
//   pub year: u32,
//   pub month: u32,
//   pub day: u32,
// }

// impl Birthdate {
//   pub fn from_grams(weight: u32) -> Self {
//     Self { grams: weight }
//   }

//   pub fn to_kilograms(&self) -> f32 {
//     (self.grams as f32) / 1000.0
//   }

//   pub fn to_pounds(&self) -> f32 {
//     (self.grams as f32) / 453.592
//   }
// }

/// A weight internally stored as an integer representing grams.
#[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
pub struct Weight {
  pub grams: u32,
}

impl Weight {
  pub fn from_grams(weight: u32) -> Self {
    Self { grams: weight }
  }

  pub fn to_kilograms(&self) -> f32 {
    (self.grams as f32) / 1000.0
  }

  pub fn to_pounds(&self) -> f32 {
    (self.grams as f32) / 453.592
  }
}

/// A height internally stored as an integer representing millimiters.
#[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
pub struct Height {
  pub millimeters: u32,
}

impl Height {
  pub fn from_millimeters(length: u32) -> Self {
    Self { millimeters: length }
  }

  pub fn to_meters(&self) -> f32 {
    (self.millimeters as f32) / 1000.0
  }

  pub fn to_inches(&self) -> f32 {
    (self.millimeters as f32) / 25.4
  }

  pub fn to_feet(&self) -> f32 {
    self.to_inches() / 12.0
  }
}

/// A registered athlete.
#[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
pub struct Athlete<AccountId> {
  /// Athlete's full name
  pub name: Vec<u8>,
  /// Account owned by the athlete.
  /// Can be reset by root authority if necessary.
  pub athlete_account: AccountId,
  /// Athlete's kind. Can be modified if necessary.
  pub kind: Vec<u8>,
  /// Athlete's sports. Can be modified if necessary.
  pub sports: Vec<u8>,
  /// Athlete's birthdate. Can be modified if necessary.
  pub birthdate: Vec<u8>,
  /// Athlete's birthplace. Can be modified if necessary.
  pub birthplace: Vec<u8>,
  /// Athlete's weight. Can be modified if necessary.
  pub weight: Weight,
  /// Applicant's height. Can be modified if necessary.
  pub height: Height,
  /// Athlete's schoolgrade. Can be updated if necessary.
  pub schoolgrade: Vec<u8>,
  /// Athlete's photo. Can be updated if necessary.
  pub photo: Option<OffchainRef>,
  /// Whether cards have been minted for this athlete.
  pub cards_minted: bool,
}

#[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
pub struct Card<AccountId, Balance> {
  pub owner: Option<AccountId>,
  pub id: CardId,
  pub tier: AthleteCardClass,
  pub value: Balance,
  pub is_on_market: bool,
}

/// Tier of athlete card.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Decode, Encode, TypeInfo)]
#[repr(u8)]
pub enum AthleteCardClass {
  /// Common tier.
  Gold,
  /// Middle tier.
  Platinum,
  /// Rarest tier.
  Diamond,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Decode, Encode, TypeInfo)]
pub struct InitialCardValues<Balance> {
  pub gold: Balance,
  pub platinum: Balance,
  pub diamond: Balance,
}

/// An application to register a person as a verified athlete.
#[derive(Clone, Debug, PartialEq, Decode, Encode, scale_info::TypeInfo)]
pub struct AthleteApplication<AccountId> {
  /// Applicant's name
  pub name: Vec<u8>,
  /// Account owned by the applicant
  pub applicant: AccountId,
  /// Applicant's kind
  pub kind: Vec<u8>,
  /// Applicant's sports
  pub sports: Vec<u8>,
  /// Applicant's birthdate
  pub birthdate: Vec<u8>,
  /// Applicant's birthplace
  pub birthplace: Vec<u8>,
  /// Applicant's schoolgrade
  pub schoolgrade: Vec<u8>,
  /// Applicant's weight
  pub weight: Weight,
  /// Applicant's height
  pub height: Height,
  /// Optional photo submitted during the application.
  /// The photo can be set or updated after registration, so this isn't mandatory.
  pub photo: Option<OffchainRef>,
}

impl<T> From<AthleteApplication<T>> for Athlete<T> {
  fn from(other: AthleteApplication<T>) -> Self {
    Athlete {
      name: other.name,
      athlete_account: other.applicant,
      kind: other.kind,
      sports: other.sports,
      birthdate: other.birthdate,
      birthplace: other.birthplace,
      schoolgrade: other.schoolgrade,
      weight: other.weight,
      height: other.height,
      photo: other.photo,
      cards_minted: false,
    }
  }
}

impl From<AthleteCardClass> for u8 {
  fn from(from: AthleteCardClass) -> Self {
    from as _
  }
}

pub fn card_hash(athlete_id: AthleteId, instance_id: InstanceId) -> [u8; 32] {
  blake2_rfc::blake2b::blake2b(
    32,
    CARD_HASH_KEY.as_bytes(),
    &(CardId { athlete_id, instance_id }).encode(),
  )
  .as_bytes()
  .try_into()
  .expect("must always be 32 bytes")
}
