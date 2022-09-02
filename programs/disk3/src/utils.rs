use anchor_lang::accounts::system_account;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    account_info::AccountInfo,
    program_memory::sol_memcmp,
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::error::NormalError;

pub const THE_AUTHOR: Pubkey =
    solana_program::pubkey!("Gcht9hSE5T9FvhpAeftRJVatwUU2aYfqNqCnS3tC3hyH");
pub const THE_AUTHOR_FEE: u64 = LAMPORTS_PER_SOL / 10000;

pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) == 0
}

// pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> Result<()> {
//     if !cmp_pubkeys(account.owner, owner) {
//         err!(BlogError::IncurrentOwner)
//     } else {
//         Ok(())
//     }
// }

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

pub fn transfer_fee<'a>(payer: &'a AccountInfo, the_author: &'a AccountInfo) -> Result<()> {
    if !cmp_pubkeys(&THE_AUTHOR, the_author.key) {
        return err!(NormalError::OnlyAuthorAccountAllowed);
    }

    // solana_program::program::invoke(
    //     &solana_program::system_instruction::transfer(payer.key, the_author.key, THE_AUTHOR_FEE),
    //     &[payer.to_account_info(), the_author.to_account_info()],
    // )?;

    // **payer.try_borrow_mut_lamports()? = payer
    //     .lamports()
    //     .checked_sub(THE_AUTHOR_FEE)
    //     .ok_or(NormalError::NotEnoughSol)?;

    // **the_author.try_borrow_mut_lamports()? = the_author
    //     .lamports()
    //     .checked_add(THE_AUTHOR_FEE)
    //     .ok_or(NormalError::NotEnoughSol)?;

    // system_instruction::transfer(&from, &to, lamports_to_send);

    Ok(())
}
