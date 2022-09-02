use anchor_lang::prelude::*;

use crate::state::{Fold, FOLD_DATA_VERSION, FOLD_SEED, RootFold, ROOT_FOLD_DATA_VERSION, ROOT_FOLD_SEED};

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
    
    /// CHECK: is not written to or read
    pub parent: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn fold_init(ctx: Context<NewFold>, name: String) -> Result<()>{
    let fold = &mut ctx.accounts.fold;
    // TODO assert owner is programID

    fold.data_version = FOLD_DATA_VERSION;
    fold.owner = ctx.accounts.payer.to_account_info().key();
    fold.parent = ctx.accounts.parent.to_account_info().key();
    fold.name = name;
    fold.counter = 0;

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
    // TODO assert manager

    fold.data_version = ROOT_FOLD_DATA_VERSION;
    fold.name = name;

    Ok(())
}
