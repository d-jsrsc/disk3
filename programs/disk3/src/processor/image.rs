use anchor_lang::prelude::*;

use crate::state::{Image, IMAGE_DATA_VERSION, IMAGE_SEED};

#[derive(Accounts)]
#[instruction(file_md5: String)]
pub struct NewImage<'info> {
    #[account(
        init, 
        payer = payer,
        space = Image::space(),
        seeds = [IMAGE_SEED, file_md5.as_bytes(), parent.key().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub image: Account<'info, Image>,

    /// CHECK: is not written to or read
    pub parent: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn image_init(ctx: Context<NewImage>, _file_md5: String, arweave_key: String) -> Result<()>{
    let file = &mut ctx.accounts.image;
    // TODO assert owner is programID

    file.data_version = IMAGE_DATA_VERSION;
    file.owner = ctx.accounts.payer.to_account_info().key();
    file.parent = ctx.accounts.parent.to_account_info().key();
    file.arweave_key = arweave_key;

    Ok(())
}