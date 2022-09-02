use anchor_lang::prelude::Pubkey;
use anchor_lang::{self, prelude::*};
use byteorder::{ByteOrder, LittleEndian};

const PUBKEY_LEN: usize = 32;
const ARWEAVE_KEY_LEN: usize = 43;
const UUID_LEN: usize = 32; // uuid without dash

pub const ROOT_FOLD_SEED: &[u8] = b"disk3-root-fold";
pub const FOLD_SEED: &[u8] = b"disk3-fold";
pub const FILE_SEED: &[u8] = b"disk3-file";
pub const IMAGE_SEED: &[u8] = b"disk3-image";
pub const VIDEO_SEED: &[u8] = b"disk3-video";

pub const ROOT_FOLD_DATA_VERSION: u8 = 1;
pub const FOLD_DATA_VERSION: u8 = 1;
pub const FILE_DATA_VERSION: u8 = 1;
pub const IMAGE_DATA_VERSION: u8 = 1;
pub const VIDEO_DATA_VERSION: u8 = 1;

pub const FILE_FOLD: Pubkey =
    solana_program::pubkey!("3CD5PpdZQUGmDw33MVFh7FriVdhXhKFRFGjPc41CRLRP");
pub const IMAGE_FOLD: Pubkey =
    solana_program::pubkey!("GGktv2GXdXm2ixmZGmwaYagXSXjzU1JQ2y9w1P1D5Be2");
pub const VIDEO_FOLD: Pubkey =
    solana_program::pubkey!("BcM6ZCGeRjoc9youESuGnYbpj9PqmKa49eDYimdNtnE3");

#[account]
pub struct RootFold {
    pub data_version: u8,
    pub name: String, // 10 * 4 // file, image, video ...
}

impl RootFold {
    pub fn space() -> usize {
        8 + 1 + 40
    }
}

#[account]
pub struct Fold {
    pub data_version: u8,
    pub owner: Pubkey,
    pub parent: Pubkey, //
    pub name: String,   // 10 * 4
    pub counter: u32,
}

impl Fold {
    pub fn space() -> usize {
        8 +                         // 8  anchor pre
        1 +                         // 1  data_version
        PUBKEY_LEN +                // 32 owner
        32 +                        // parent
        4 + 40 +                    // 10 * 4 name
        4 // counter
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Result<Self> {
        let data = account_info.data.borrow();
        let data = data.as_ref();
        let mut cursor = 8;

        // data_version
        cursor += 1;

        // owner
        let owner = &data[cursor..cursor + PUBKEY_LEN];
        cursor += PUBKEY_LEN;
        let owner = Pubkey::try_from_slice(owner)?;

        // parent
        let parent = &data[cursor..cursor + PUBKEY_LEN];
        cursor += PUBKEY_LEN;
        let parent = Pubkey::try_from_slice(parent)?;

        // name
        let key_pre = &data[cursor..cursor + 4];
        cursor += 4;

        let len = LittleEndian::read_uint(key_pre, 4) as usize;
        let name = &data[cursor..cursor + len];
        cursor += len;

        let name = String::from_utf8_lossy(name).to_string();

        // counter
        let counter: &[u8; 4] = &data[cursor..cursor + 4].try_into().unwrap(); // TODO
        let counter: u32 = u32::from_le_bytes(*counter);

        Ok(Fold {
            data_version: FOLD_DATA_VERSION,
            owner,
            parent,
            name,
            counter,
        })
    }
}

// EncryptedEnum == 0: noEncrypted, 1: encryptedWithWallet, 2: encryptedCustom
#[account]
pub struct File {
    pub data_version: u8,
    pub owner: Pubkey,
    pub parent: Pubkey, // file, image, video, pubkeyString
    pub arweave_key: String,
    pub encrypted: u8, // 0: noEncrypted, 1: encryptedWithWallet, 2: encryptedCustom
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
        32 +                    // 32 parent
        4 + 43 + // arweaveKey
        1
    }
}

#[account]
pub struct Image {
    pub data_version: u8,
    pub owner: Pubkey,
    pub parent: Pubkey, // file, image, video, pubkeyString
    pub arweave_key: String,
    pub encrypted: u8,
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
        32 +                        // 32 parent
        4 + 43 + // arweaveKey
        1
    }
}

#[account]
pub struct Video {
    pub data_version: u8,
    pub owner: Pubkey,
    pub parent: Pubkey, // file, image, video, pubkeyString
    pub arweave_key: String,
    pub encrypted: u8,
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
        32 +                        // parent
        4 + 43 + // arweaveKey
        1
    }
}
