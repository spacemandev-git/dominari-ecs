use anchor_lang::prelude::*;

#[error_code]
pub enum DominariError {
    #[msg("Entity must be empty!")]
    InvalidEntity,
}

#[error_code]
pub enum ComponentErrors {
    #[msg("Invalid Owner!")]
    InvalidOwner,
}