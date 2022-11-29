use anchor_lang::prelude::*;

#[error_code]
pub enum DominariError {
    #[msg("Entity must be empty!")]
    InvalidEntity,

    #[msg("Instance already has max players!")]
    PlayerCountExceeded,
}

#[error_code]
pub enum ComponentErrors {
    #[msg("Invalid Owner!")]
    InvalidOwner,

    #[msg("String too long!")]
    StringTooLong,
}
