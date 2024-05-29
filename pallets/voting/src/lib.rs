#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        VoteCast(T::AccountId, u32),
    }

    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn cast_vote(origin: OriginFor<T>, proposal_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Votes::<T>::insert(&who, proposal_id);
            Self::deposit_event(Event::VoteCast(who, proposal_id));
            Ok(())
        }
    }
}
