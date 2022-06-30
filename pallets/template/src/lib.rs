#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

//use pallet_reputation;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::sp_runtime::ArithmeticError;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}
	type AccountId<T> = <T as frame_system::Config>::AccountId;
	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	sadfasdf

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen,TypeInfo)]
    pub struct Space<AccountId> {
        pub owner: AccountId,
        pub reputation : u32,
    }

	#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug,TypeInfo)]
	pub enum ScoringAction {
		CreateModeration=1,
		CreateMovie=2,
		CreateFestival=3,
		AddMovieToFesival=4,
		VoteForMovieInFestivals=5,
		CreateRankingList=6,
		AddMovieToRankingList=7,
		VoteForMovieInRankingList=8,
		CreatePost=9,
		CommentOnPost=10,
		CommentOnComment=11,
		FollowAccount=12,
		UpvoteMovie=13,
		DownvoteMovie=14,
		UpvoteComment=15,
		DownvoteComment=16,
		//Follow=1, Space and Account
	}

	impl Default for ScoringAction {
	    fn default() -> Self {
	        ScoringAction::FollowAccount
	    }
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug,TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct SocialAccount<> {
		pub followers_count: u32,
		pub following_accounts_count: u16,
		pub following_spaces_count: u16,
		pub reputation: u32,
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug,TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Profile<T: Config> {
		pub created: WhoAndWhen<T>,
		pub updated: Option<WhoAndWhen<T>>,
		pub content: Content
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug,TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct WhoAndWhen<T:Config> {
		pub account: T::AccountId,
		pub block: T::BlockNumber,
		pub time: T::Moment,
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug,TypeInfo)]
	pub enum Content {
	    /// No content.
	    None,
	}

	pub type ReputationScore = u32;
	
	pub type ReputationValue = u32;

	#[pallet::storage]
	pub type SpaceReputation<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn members_score)]
	pub type MemberScore<T> = StorageMap<
		_,
		Blake2_128Concat, 
		AccountId<T>, 
		(ReputationScore, u32), 
		ValueQuery
		>;

	#[pallet::storage]
	#[pallet::getter(fn repu_score)]
	pub type RepuScore<T> = StorageMap<
		_,
		Blake2_128Concat, 
		AccountId<T>, 
		ReputationValue, 
		ValueQuery
		>;

	#[pallet::storage]
	#[pallet::getter(fn teste_storage)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type TesteStorage<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn social_account_by_id)]
	pub type SocialAccountById<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, SocialAccount>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},	
			}
		}
	}

	impl<T: Config> Pallet<T> {

   
		pub fn get_or_new_social_account(account: T::AccountId) -> SocialAccount {
			Self::social_account_by_id(account).unwrap_or(
				SocialAccount {
					followers_count: 0,
					following_accounts_count: 0,
					following_spaces_count: 0,
					reputation: 1,
					
				}
			)
		}
	}

	impl<T: Config> Pallet<T> {

		pub fn action_performed(
			acc: T::AccountId,
			action: ScoringAction,
		) -> Result<u32, DispatchError>  {
	
			let mut social_account = Pallet::<T>::get_or_new_social_account(acc.clone());
			
			
			//pallet_reputation::pallet::Pallet::<T>::action_reputation;

			//add reputation
			if action == ScoringAction::CreatePost{
				social_account.reputation+=5;
	
			};
			
			social_account.reputation+=action as u32;
			SocialAccountById::<T>::insert(acc.clone(), social_account.clone());
			//pallet_profile::SocialAccountById::<T>::insert(acc.clone(), social_account.clone());
	
	
			Ok(action as u32)
		  
		}
	}
	

	impl<T: Config> Pallet<T> {

		pub fn action_teste(
			
			teste: u32,
		) -> DispatchResult  {
	
			
			//add reputation
			<TesteStorage<T>>::put(teste);
			
			
			Ok(())
		  
		}
	
	}


}
