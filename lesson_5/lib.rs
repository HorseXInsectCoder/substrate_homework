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
