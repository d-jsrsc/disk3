mod error;
mod processor;
mod state;
mod utils;

use anchor_lang::prelude::*;
use processor::*;

declare_id!("disk7ooFCfjVoGxsjYBSxthgbEKJGRmaXbiFicyepGF");

#[program]
pub mod disk3 {

    use super::*;

    pub fn init_file<'info>(
        ctx: Context<'_, '_, '_, 'info, NewFile<'info>>,
        file_md5: String,
        arweave_key: String,
        encrypted: u8,
    ) -> Result<()> {
        file_init(ctx, file_md5, arweave_key, encrypted)
    }

    pub fn init_image(ctx: Context<NewImage>, file_md5: String, arweave_key: String) -> Result<()> {
        image_init(ctx, file_md5, arweave_key)
    }

    pub fn init_video(ctx: Context<NewVideo>, file_md5: String, arweave_key: String) -> Result<()> {
        video_init(ctx, file_md5, arweave_key)
    }

    pub fn init_fold(ctx: Context<NewFold>, name: String) -> Result<()> {
        fold_init(ctx, name)
    }

    pub fn init_root_fold(ctx: Context<NewRootFold>, name: String) -> Result<()> {
        root_fold_init(ctx, name)
    }
}
