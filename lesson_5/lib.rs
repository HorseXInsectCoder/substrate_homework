#![cfg_attr(not(feature = "std"), no_std)]

/// A module for proof of the existence
// pub use frame_system::pallet::*;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // 承载功能模块的pallet
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // 存储单元
    #[pallet::storage]
    #[pallet::getter(fn proofs)]    // 使用宏来定义了一个getter函数，叫proofs，即会触发被宏标注的代码
    // (T::AccountId, T::BlockNumber)，AccountId表示用户ID，BlockNumber表示存入存证的区块，这两个类型都来自系统模块
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber)
    >;

    // #[pallet::metadata(T::AccountId = "AccountId")]          // 转换给前端
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        ClaimTransfered(T::AccountId, Vec<u8>, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// 存证已经存在，已经被创建
        ProofAlreadyExist,
        /// 存证不存在，无法撤销
        ClaimNotExist,
        /// 该存证是由另外一个用户创建，当前账户无权处理
        NotClaimOwner,
    }

    // 模块定义里有一些特殊的函数可以在区块的某一个时间执行，这些特殊的函数定义在Hooks里面
    // #[pallet::hooks]
    // impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    //     // 这里没有想要定义的特殊函数，所以为空
    // }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // 创建存证的可调用函数
        // origin表示发送方，claim表示存证的Hash值
        #[pallet::weight(0)]    // 实际情况是weight必须先测试得到合理的值，并且weight的选取是和存储单元有直接关系
        pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            // 校验发送方，并且在校验完成后获取发送方的ID
            let sender = ensure_signed(origin)?;

            // 如果不存在存证，就返回错误
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            // 存储记录
            Proofs::<T>::insert(
                &claim,
                // 第一个元素是发送者（存证的Owner）,第二个元素是区块
                (sender.clone(),frame_system::Pallet::<T>::block_number()),
            );

            // 插入成功，触发事件
            Self::deposit_event(Event::ClaimCreated(sender, claim));

            Ok(().into())
        }

        // 吊销存证
        #[pallet::weight(0)]
        pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            // 查看存证值是否存在
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            // 只有Owner才可以吊销
            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(().into())
        }

        // 转移存证
        #[pallet::weight(0)]
        pub fn transfer_claim(origin: OriginFor<T>, claim: Vec<u8>, recv_account: T::AccountId) -> DispatchResultWithPostInfo {
            // 检查发送方是否合法
            let sender = ensure_signed(origin)?;

            // 检查存证是否存在
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
            let (owner, _) = Proofs::<T>::get(&claim).unwrap();

            // 检查sender是否为owner
            ensure!(sender == owner, Error::<T>::NotClaimOwner);

            // 开始转移
            let current_block = <frame_system::Pallet<T>>::block_number();
            Proofs::<T>::mutate(&claim, |value| {
                value.as_mut().unwrap().0 = recv_account.clone();
                value.as_mut().unwrap().1 = current_block;
            });

            Self::deposit_event(Event::ClaimTransfered(sender, claim, recv_account));
            Ok(().into())
        }
    }
}
