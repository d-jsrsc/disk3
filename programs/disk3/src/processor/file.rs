use anchor_lang::{prelude::*, system_program};

use crate::{
    state::{File, FILE_DATA_VERSION, FILE_SEED},
    error::NormalError,
    utils::{THE_AUTHOR_FEE, cmp_pubkeys, dec_fold_counter, inc_fold_counter}
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
    /// CHECK: written
    #[account(mut)]
    pub parent: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn file_init<'info>(ctx: Context<'_, '_, '_, 'info, NewFile<'info>>, _file_md5: String, arweave_key: String, encrypted: u8) -> Result<()>{
    if !cmp_pubkeys(ctx.accounts.payer.owner, &system_program::ID) {
        return err!(NormalError::OnlySystemAccountAllowed);
    }

    let file = &mut ctx.accounts.file;

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
    
    let fold_account_info = &ctx.accounts.parent.to_account_info();
    inc_fold_counter(fold_account_info)?;

    Ok(())
}


#[derive(Accounts)]
#[instruction(file_md5: String)]
pub struct DelFile<'info> {
    #[account( 
        mut,
        close = payer,
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
}

pub fn file_del(ctx: Context<DelFile>, _file_md5: String) -> Result<()> {
    let file = &ctx.accounts.file;
    if !cmp_pubkeys(&file.owner, ctx.accounts.payer.to_account_info().key) {
        return err!(NormalError::IncurrentOwner);
    };

    let fold_account_info = &ctx.accounts.parent.to_account_info();
    dec_fold_counter(fold_account_info)?;

    Ok(())
}