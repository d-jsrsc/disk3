use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    account_info::AccountInfo,
    program_memory::sol_memcmp,
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::error::NormalError;
use crate::state::{Fold, FILE_FOLD, IMAGE_FOLD, VIDEO_FOLD};

pub const THE_AUTHOR: Pubkey =
    solana_program::pubkey!("Gcht9hSE5T9FvhpAeftRJVatwUU2aYfqNqCnS3tC3hyH");
pub const THE_AUTHOR_FEE: u64 = LAMPORTS_PER_SOL / 10000;

pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) == 0
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> Result<()> {
    msg!("{:?} {:?}", account.owner.to_string(), owner.to_string());
    if !cmp_pubkeys(account.owner, owner) {
        err!(NormalError::IncurrentOwner)
    } else {
        Ok(())
    }
}

pub fn puffed_out_string(s: &str, size: usize) -> String {
    let mut array_of_zeroes = vec![];
    let puff_amount = size - s.len();
    while array_of_zeroes.len() < puff_amount {
        array_of_zeroes.push(0u8);
    }
    s.to_owned() + std::str::from_utf8(&array_of_zeroes).unwrap()
}

// pub fn assert_initialized<T: Pack + IsInitialized>(account_info: &AccountInfo) -> Result<T> {
//     let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
//     if !account.is_initialized() {
//         err!(BlogError::CollectionNotVerified)
//     } else {
//         Ok(account)
//     }
// }

pub fn inc_fold_counter(fold_account_info: &AccountInfo) -> Result<()> {
    // if is parent is not root-fold
    if fold_account_info.key.eq(&FILE_FOLD)
        || fold_account_info.key.eq(&IMAGE_FOLD)
        || fold_account_info.key.eq(&VIDEO_FOLD)
    {
        return Ok(());
    }
    let mut fold: Fold = Fold::from_account_info(fold_account_info)?;
    fold.counter += 1;
    let mut w = &mut fold_account_info.data.borrow_mut()[..];
    fold.try_serialize(&mut w)?;
    Ok(())
}

pub fn dec_fold_counter(fold_account_info: &AccountInfo) -> Result<()> {
    if fold_account_info.key.eq(&FILE_FOLD)
        || fold_account_info.key.eq(&IMAGE_FOLD)
        || fold_account_info.key.eq(&VIDEO_FOLD)
    {
        return Ok(());
    }
    let mut fold: Fold = Fold::from_account_info(fold_account_info)?;
    fold.counter -= 1;
    let mut w = &mut fold_account_info.data.borrow_mut()[..];
    fold.try_serialize(&mut w)?;
    Ok(())
}
