use anchor_lang::prelude::*;

use crate::state::{File, FILE_DATA_VERSION, FILE_SEED, Fold, FILE_FOLD, IMAGE_FOLD, VIDEO_FOLD};

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


pub fn file_init(ctx: Context<NewFile>, _file_md5: String, arweave_key: String) -> Result<()>{
    let file = &mut ctx.accounts.file;
    // TODO assert owner is programID

    file.data_version = FILE_DATA_VERSION;
    file.owner = ctx.accounts.payer.to_account_info().key();
    file.parent = ctx.accounts.parent.to_account_info().key();
    file.arweave_key = arweave_key;

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