#![cfg_attr(not(feature = "std"), no_std)]
//A module for proof of extisence
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet{
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, 
        pallet_prelude::* 
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config:frame_system::Config 
    {
        type Event:From <Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber)
    >;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config>{        
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRemoved(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T>{      
        ProofAlreadyExist,
        ClaimNotExist,
        ClaimNotClaimOwner
    }
    
    #[pallet::hooks]
    impl <T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>{}

    #[pallet::call]
    impl<T:Config> Pallet<T>{
        //定义创建存证的可调用函数
        #[pallet::weight(0)]
        pub fn create_claim(
            origin:OriginFor<T>,
            claim: Vec<u8>
        ) -> DispatchResultWithPostInfo{
            let sender = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            Proofs::<T>::insert(
                &claim, 
                (sender.clone(), frame_system::Pallet::<T>::block_number())
            );
            
            Self::deposit_event(Event::ClaimCreated(sender,claim));
            Ok(().into())
        }

            //吊销存证
            #[pallet::weight(0)]
            pub fn revoke_claim(
                origin: OriginFor<T>,
                claim: Vec<u8>
            ) -> DispatchResultWithPostInfo{
                let sender = ensure_signed(origin)?;

                let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

                ensure!(owner == sender, Error::<T>::ClaimNotClaimOwner);

                Proofs::<T>::remove(&claim);
                
                Self::deposit_event(Event::ClaimRemoved(sender,claim));
                Ok(().into())
            }
       
    }

   

}