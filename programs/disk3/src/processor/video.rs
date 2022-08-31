use anchor_lang::prelude::*;

use crate::state::{Video, VIDEO_DATA_VERSION, VIDEO_SEED};

#[derive(Accounts)]
#[instruction(file_md5: String, parent: String)]
pub struct NewVideo<'info> {
    #[account(
        init, 
        payer = payer,
        space = Video::space(),
        seeds = [VIDEO_SEED, file_md5.as_bytes(), parent.as_bytes(), payer.key().as_ref()],
        bump
    )]
    pub video: Account<'info, Video>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn video_init(ctx: Context<NewVideo>, _file_md5: String, parent: String, arweave_key: String) -> Result<()>{
    let video = &mut ctx.accounts.video;
    // TODO assert owner is programID

    video.data_version = VIDEO_DATA_VERSION;
    video.owner = ctx.accounts.payer.to_account_info().key();
    video.parent = parent;
    video.arweave_key = arweave_key;

    Ok(())
}