use anchor_lang::prelude::*;

#[error_code]
pub enum NormalError {
    #[msg("OnlySystemAccountAllowed")]
    OnlySystemAccountAllowed,
    #[msg("OnlyAuthorAccountAllowed")]
    OnlyAuthorAccountAllowed,
    #[msg("FileEncryptedErr")]
    FileEncryptedErr,
    #[msg("NumericalOverflowError")]
    NumericalOverflowError,
    #[msg("NotEnoughSol")]
    NotEnoughSol,
    #[msg("IncurrentOwner")]
    IncurrentOwner,
    #[msg("FoldNotEmpty")]
    FoldNotEmpty,
}
