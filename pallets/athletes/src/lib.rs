#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod macros;
mod crypto;
mod mock;
mod offchain;
mod tests;

pub use crypto::*;
pub use offchain::*;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use alloc::vec::Vec;
  use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    ensure,
    pallet_prelude::{StorageDoubleMap, StorageMap, StorageValue},
    traits::{
      tokens::nonfungibles::{Create, Inspect, InspectEnumerable, Mutate, Transfer},
      Currency, ExistenceRequirement, Get, Hooks, IsType, WithdrawReasons,
    },
    Twox64Concat,
  };
  use frame_system::{
    ensure_root, ensure_signed,
    offchain::{AppCrypto, SendTransactionTypes, SigningTypes},
    pallet_prelude::{BlockNumberFor, OriginFor},
  };
  use meta_athlete_primitives::{
    card_hash, Athlete, AthleteApplication, AthleteCardClass, AthleteId, Card, CardId,
    InitialCardValues, InstanceId,
  };
  use parity_scale_codec::Encode;
  use sp_core::H256;
  use sp_runtime::ArithmeticError;

  type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

  #[pallet::config]
  pub trait Config: SendTransactionTypes<Call<Self>> + SigningTypes + frame_system::Config {
    #[pallet::constant]
    type SystemAccountId: Get<Self::AccountId>;

    #[pallet::constant]
    type InitialCardValues: Get<InitialCardValues<BalanceOf<Self>>>;

    type Card: Create<Self::AccountId, ClassId = AthleteId, InstanceId = InstanceId>
      + InspectEnumerable<Self::AccountId, ClassId = AthleteId, InstanceId = InstanceId>
      + Mutate<Self::AccountId, ClassId = AthleteId, InstanceId = InstanceId>
      + Transfer<Self::AccountId, ClassId = AthleteId, InstanceId = InstanceId>;
    type Currency: Currency<Self::AccountId>;
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    type OffchainAuthority: AppCrypto<Self::Public, Self::Signature>;
    type OffchainUnsignedGracePeriod: Get<Self::BlockNumber>;
    type OffchainUnsignedInterval: Get<Self::BlockNumber>;
  }

  #[pallet::error]
  pub enum Error<T> {
    AthleteAlreadyExists,
    CardAttributeDoesNotExist,
    CardDoesNotHaveAnOwner,
    CardIsNotForSale,
    MustBeCardOwner,
    InvalidApplicationId,
    InvalidAthleteId,
    CardsAlreadyMinted,
    InvalidCardHash,
    InsufficientFunds,
    CouldNotDeposit,
  }

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    AthleteApplicationSubmitted { id: AthleteId, application: AthleteApplication<T::AccountId> },
    AthleteApplicationApproved { id: AthleteId },
    CardMinted { card_hash: H256, card_id: CardId, tier: AthleteCardClass },
    CardSold { card_hash: H256, card_id: CardId, who: T::AccountId, value: BalanceOf<T> },
    CardBought { card_hash: H256, card_id: CardId, who: T::AccountId, value: BalanceOf<T> },
  }

  #[pallet::pallet]
  pub struct Pallet<T>(_);

  #[pallet::storage]
  pub type AthleteCounter<T: Config> = StorageValue<_, AthleteId>;

  #[pallet::storage]
  #[pallet::getter(fn athletes)]
  pub type Athletes<T: Config> = StorageMap<_, Twox64Concat, AthleteId, Athlete<T::AccountId>>;

  #[pallet::storage]
  #[pallet::getter(fn cards)]
  pub type Cards<T: Config> = StorageMap<_, Twox64Concat, H256, Card<T::AccountId, BalanceOf<T>>>;

  #[pallet::storage]
  pub type Applications<T: Config> =
    StorageMap<_, Twox64Concat, AthleteId, AthleteApplication<T::AccountId>>;

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[frame_support::transactional]
    #[pallet::weight(10_000)]
    pub fn submit_athlete_application(
      origin: OriginFor<T>,
      application: AthleteApplication<T::AccountId>,
    ) -> DispatchResult {
      // TODO: Demand deposit from registrar to discourage spam?
      ensure_signed(origin)?;

      let id = Self::next_athlete_id()?;
      <Applications<T>>::insert(id, &application);

      Self::deposit_event(Event::<T>::AthleteApplicationSubmitted { id, application });

      Ok(())
    }

    #[frame_support::transactional]
    #[pallet::weight(10_000)]
    pub fn approve_application(origin: OriginFor<T>, athlete_id: AthleteId) -> DispatchResult {
      ensure_signed(origin)?;

      let application =
        <Applications<T>>::try_get(athlete_id).map_err(|_| Error::<T>::InvalidApplicationId)?;

      let athlete: Athlete<T::AccountId> = application.into();
      <Applications<T>>::remove(athlete_id);
      <Athletes<T>>::insert(athlete_id, &athlete);

      Self::deposit_event(Event::<T>::AthleteApplicationApproved { id: athlete_id });

      Ok(())
    }

    #[frame_support::transactional]
    #[pallet::weight(10_000)]
    pub fn mint_cards(origin: OriginFor<T>, athlete_id: AthleteId) -> DispatchResult {
      let sender = ensure_signed(origin)?;
      let mut athlete =
        <Athletes<T>>::try_get(athlete_id).map_err(|_| Error::<T>::InvalidAthleteId)?;

      ensure!(!athlete.cards_minted, Error::<T>::CardsAlreadyMinted);

      let system_id = Self::system_account_id();
      T::Card::create_class(&athlete_id, &system_id, &system_id)?;

      let add_instances = |range, tier| {
        for instance_id in range {
          let value = match tier {
            AthleteCardClass::Gold => <T::InitialCardValues>::get().gold,
            AthleteCardClass::Platinum => <T::InitialCardValues>::get().platinum,
            AthleteCardClass::Diamond => <T::InitialCardValues>::get().diamond,
          };

          let card_id = CardId { athlete_id, instance_id };
          let card_hash = card_hash(athlete_id, instance_id);
          let card_hash = H256(card_hash);
          <Cards<T>>::insert(
            card_hash,
            Card { owner: Some(sender.clone())/*None*/, id: card_id, tier, value, is_on_market: true },
          );

          Self::deposit_event(Event::<T>::CardMinted { card_id, card_hash, tier });
        }
        DispatchResult::Ok(())
      };

      add_instances(60..160, AthleteCardClass::Gold)?;
      add_instances(10..60, AthleteCardClass::Platinum)?;
      add_instances(0..10, AthleteCardClass::Diamond)?;

      athlete.cards_minted = true;
      <Athletes<T>>::insert(athlete_id, athlete);

      Ok(())
    }

    #[frame_support::transactional]
    #[pallet::weight(10_000)]
    pub fn buy_card(origin: OriginFor<T>, card_hash: H256) -> DispatchResult {
      let who = ensure_signed(origin)?;
      let mut card = <Cards<T>>::get(card_hash).ok_or(Error::<T>::InvalidCardHash)?;

      ensure!(card.is_on_market, Error::<T>::CardIsNotForSale);

      let imbalance = <T::Currency>::burn(card.value);
      <T::Currency>::settle(
        &who,
        imbalance,
        WithdrawReasons::TRANSFER,
        ExistenceRequirement::KeepAlive,
      )
      .map_err(|_| Error::<T>::InsufficientFunds)?;

      card.is_on_market = true;
      card.owner = Some(who.clone());

      Self::deposit_event(Event::<T>::CardBought {
        card_id: card.id,
        card_hash,
        who,
        value: card.value,
      });

      <Cards<T>>::insert(card_hash, card);

      Ok(())
    }

    #[frame_support::transactional]
    #[pallet::weight(10_000)]
    pub fn sell_card(origin: OriginFor<T>, card_hash: H256) -> DispatchResult {
      let who = ensure_signed(origin)?;
      let mut card = <Cards<T>>::get(card_hash).ok_or(Error::<T>::InvalidCardHash)?;

      ensure!(
        card.owner.as_ref().map(|owner| owner == &who).unwrap_or(false),
        Error::<T>::MustBeCardOwner
      );

      let imbalance = <T::Currency>::issue(card.value);
      <T::Currency>::resolve_into_existing(&who, imbalance)
        .map_err(|_| Error::<T>::CouldNotDeposit)?;

      card.is_on_market = true;
      card.owner = None;

      Self::deposit_event(Event::<T>::CardSold {
        card_id: card.id,
        card_hash,
        who,
        value: card.value,
      });

      <Cards<T>>::insert(card_hash, card);

      Ok(())
    }
  }

  impl<T> Pallet<T>
  where
    T: Config,
  {
    pub(crate) fn system_account_id() -> T::AccountId {
      T::SystemAccountId::get()
    }

    pub(crate) fn owner(
      class_id: &AthleteId,
      instance_id: &InstanceId,
    ) -> Result<T::AccountId, DispatchError> {
      T::Card::owner(class_id, instance_id).ok_or_else(|| Error::<T>::CardDoesNotHaveAnOwner.into())
    }

    fn next_athlete_id() -> Result<AthleteId, DispatchError> {
      let id = if let Ok(current) = AthleteCounter::<T>::try_get() {
        current.checked_add(1u64).ok_or(ArithmeticError::Overflow)?
      } else {
        1
      };
      <AthleteCounter<T>>::put(id);
      Ok(id)
    }
  }
}
