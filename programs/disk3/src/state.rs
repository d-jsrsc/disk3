use anchor_lang::prelude::*;

const PUBKEY_LEN: usize = 32;
const ARWEAVE_KEY_LEN: usize = 43;
const UUID_LEN: usize = 32; // uuid without dash

pub const FOLD_SEED: &[u8] = b"disk3-fold";
pub const FILE_SEED: &[u8] = b"disk3-file";
pub const IMAGE_SEED: &[u8] = b"disk3-image";
pub const VIDEO_SEED: &[u8] = b"disk3-video";

pub const FOLD_DATA_VERSION: u8 = 1;
pub const FILE_DATA_VERSION: u8 = 1;
pub const IMAGE_DATA_VERSION: u8 = 1;
pub const VIDEO_DATA_VERSION: u8 = 1;

#[account]
pub struct Fold {
    pub data_version: u8,
    pub owner: Pubkey,
    pub name: String,   // 10 * 4
    pub parent: String, // file, image, video, pubkeyString
    pub counter: u32,
}

impl Fold {
    pub fn space() -> usize {
        8 +                         // 8  anchor pre
        1 +                         // 1  data_version
        PUBKEY_LEN +                // 32 owner
        4 + 40 +                    // 10 * 4 name
        4 + 32 +                    // pubkeyString
        4 // counter
    }
}

#[account]
pub struct File {
    pub data_version: u8,
    pub owner: Pubkey,
    pub parent: String, // file, image, video, pubkeyString
    pub arweave_key: String,
}

// fileMetadata
// {
//     fileName:
//     fileMd5:
//     fileType:
//     fileSize:
//     file:
//     encrypted:
// }

impl File {
    pub fn space() -> usize {
        8 +                         // 8  anchor pre
        1 +                         // 1  data_version
        PUBKEY_LEN +                // 32 owner
        4 + 32 +                    // 4 + 32 parent
        4 + 43 // arweaveKey
    }
}

#[account]
pub struct Image {
    pub data_version: u8,
    pub owner: Pubkey,
    pub parent: String, // file, image, video, pubkeyString
    pub arweave_key: String,
}

// fileMetadata
// {
//     fileName:
//     fileMd5:
//     fileType:
//     fileSize:
//     file:
//     encrypted:
// }

impl Image {
    pub fn space() -> usize {
        8 +                         // 8  anchor pre
        1 +                         // 1  data_version
        PUBKEY_LEN +                // 32 owner
        4 + 32 +                    // 4 + 32 parent
        4 + 43 // arweaveKey
    }
}

#[account]
pub struct Video {
    pub data_version: u8,
    pub owner: Pubkey,
    pub parent: String, // file, image, video, pubkeyString
    pub arweave_key: String,
}

// fileMetadata
// {
//     fileName:
//     fileMd5:
//     fileType:
//     fileSize:
//     file:
//     encrypted:
// }

impl Video {
    pub fn space() -> usize {
        8 +                         // 8  anchor pre
        1 +                         // 1  data_version
        PUBKEY_LEN +                // 32 owner
        4 + 32 +                    // parent
        4 + 43 // arweaveKey
    }
}
