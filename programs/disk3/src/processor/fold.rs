use anchor_lang::{prelude::*, system_program};

use crate::error::NormalError;
use crate::utils::{inc_fold_counter, dec_fold_counter};
use crate::{state::{Fold, FOLD_DATA_VERSION, FOLD_SEED, RootFold, ROOT_FOLD_DATA_VERSION, ROOT_FOLD_SEED}, utils::{cmp_pubkeys, THE_AUTHOR}};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct NewFold<'info> {
    #[account(
        init, 
        payer = payer,
        space = Fold::space(),
        seeds = [FOLD_SEED, name.as_bytes(), parent.key().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub fold: Account<'info, Fold>,
    
    /// CHECK: written
    #[account(mut)]
    pub parent: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn fold_init(ctx: Context<NewFold>, name: String) -> Result<()>{
    if !cmp_pubkeys(ctx.accounts.payer.owner, &system_program::ID) {
        return err!(NormalError::OnlySystemAccountAllowed);
    }

    let fold = &mut ctx.accounts.fold;
    fold.data_version = FOLD_DATA_VERSION;
    fold.owner = ctx.accounts.payer.to_account_info().key();
    fold.parent = ctx.accounts.parent.to_account_info().key();
    fold.name = name;
    fold.counter = 0;

    let fold_account_info = &ctx.accounts.parent.to_account_info();
    inc_fold_counter(fold_account_info)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DelFold<'info> {
    #[account( 
        mut,
        close = payer,
        seeds = [FOLD_SEED, name.as_bytes(), parent.key().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub fold: Account<'info, Fold>,
    /// CHECK: 
    #[account(mut)]
    pub parent: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn fold_del(ctx: Context<DelFold>, _name: String) -> Result<()> {
    let fold = &ctx.accounts.fold;
    if fold.counter > 0 {
        return err!(NormalError::FoldNotEmpty);
    }
    if !cmp_pubkeys(&fold.owner, ctx.accounts.payer.to_account_info().key) {
        return err!(NormalError::IncurrentOwner);
    };

    let fold_account_info = &ctx.accounts.parent.to_account_info();
    dec_fold_counter(fold_account_info)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct NewRootFold<'info> {
    #[account(
        init, 
        payer = payer,
        space = Fold::space(),
        seeds = [ROOT_FOLD_SEED, name.as_bytes()],
        bump
    )]
    pub fold: Account<'info, RootFold>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn root_fold_init(ctx: Context<NewRootFold>, name: String) -> Result<()>{
    let fold = &mut ctx.accounts.fold;
    if !cmp_pubkeys(ctx.accounts.payer.to_account_info().key, &THE_AUTHOR) {
        return err!(NormalError::IncurrentOwner);
    }

    fold.data_version = ROOT_FOLD_DATA_VERSION;
    fold.name = name;

    Ok(())
}
