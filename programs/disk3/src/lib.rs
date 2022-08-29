mod processor;
mod state;

use anchor_lang::prelude::*;
use processor::*;
declare_id!("Disk39nVau5sgCmr6oJuKdfg2Nc5VQpXgd5C4ztSt7Vd");

#[program]
pub mod disk3 {

    use super::*;

    pub fn init_file(
        ctx: Context<NewFile>,
        file_md5: String,
        parent: String,
        arweave_key: String,
    ) -> Result<()> {
        file_init(ctx, file_md5, parent, arweave_key)
    }

    pub fn init_image(
        ctx: Context<NewImage>,
        file_md5: String,
        parent: String,
        arweave_key: String,
    ) -> Result<()> {
        image_init(ctx, file_md5, parent, arweave_key)
    }
}
