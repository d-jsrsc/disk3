use anchor_lang::prelude::*;

use crate::state::{File, FILE_DATA_VERSION, FILE_SEED};

#[derive(Accounts)]
#[instruction(file_md5: String, parent: String)]
pub struct NewFile<'info> {
    #[account(
        init, 
        payer = payer,
        space = File::space(),
        seeds = [FILE_SEED, file_md5.as_bytes(), parent.as_bytes(), payer.key().as_ref()],
        bump
    )]
    pub file: Account<'info, File>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn file_init(ctx: Context<NewFile>, _file_md5: String, parent: String, arweave_key: String) -> Result<()>{
    let file = &mut ctx.accounts.file;
    // TODO assert owner is programID

    file.data_version = FILE_DATA_VERSION;
    file.owner = ctx.accounts.payer.to_account_info().key();
    file.parent = parent;
    file.arweave_key = arweave_key;

    Ok(())
}