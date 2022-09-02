use std::borrow::Borrow;

use anchor_lang::prelude::*;
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::{
    state::{File, FILE_DATA_VERSION, FILE_SEED, Fold, FILE_FOLD, IMAGE_FOLD, VIDEO_FOLD},
    error::NormalError,
    utils::{transfer_fee, THE_AUTHOR_FEE}
};

#[derive(Accounts)]
#[instruction(file_md5: String)]
pub struct NewFile<'info> {
    #[account(
        init, 
        payer = payer,
        space = File::space(),
        seeds = [FILE_SEED, file_md5.as_bytes(), parent.key().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub file: Account<'info, File>,
    /// CHECK: 
    #[account(mut)]
    pub parent: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn file_init<'info>(ctx: Context<'_, '_, '_, 'info, NewFile<'info>>, _file_md5: String, arweave_key: String, encrypted: u8) -> Result<()>{
    let file = &mut ctx.accounts.file;
    // TODO assert owner is programID

    file.data_version = FILE_DATA_VERSION;
    file.owner = ctx.accounts.payer.to_account_info().key();
    file.parent = ctx.accounts.parent.to_account_info().key();
    file.arweave_key = arweave_key;
    file.encrypted = encrypted;

    let encrypted = encrypted == 1 || encrypted == 2;
    if ctx.remaining_accounts.is_empty() == encrypted {
        return err!(NormalError::FileEncryptedErr)
    }


    if !ctx.remaining_accounts.is_empty() {        
        // https://stackoverflow.com/questions/72807527/is-it-possible-to-transfer-tokens-to-account-declared-in-remaining-accounts
        let payer_account = &ctx.accounts.payer.to_account_info();
        let the_author_account = &ctx.remaining_accounts[0];   

        solana_program::program::invoke(&solana_program::system_instruction::transfer(
            payer_account.key,
            the_author_account.key,
            THE_AUTHOR_FEE,
        ), &[
            payer_account.clone(),
            the_author_account.clone(),
        ])?;
    }


    // if is parent is not root-fold 
    let fold_account_info = &ctx.accounts.parent.to_account_info();
    if fold_account_info.key.eq(&FILE_FOLD) || 
       fold_account_info.key.eq(&IMAGE_FOLD) || 
       fold_account_info.key.eq(&VIDEO_FOLD) {
        return Ok(())
    }
    let mut fold: Fold = Fold::from_account_info(fold_account_info)?;
    fold.counter += 1;
    let mut w = &mut fold_account_info.data.borrow_mut()[..];
    fold.try_serialize(&mut w)?;

    Ok(())
}