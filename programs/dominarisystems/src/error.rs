use anchor_lang::prelude::*;

#[error_code]
pub enum DominariError {
    #[msg("Entity must be empty!")]
    InvalidEntity,
}